#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;

pub mod errors;

use std::{convert::TryInto, io};

use actix_web::{error::BlockingError, web, Error};
use actix_web::{App, HttpResponse, HttpServer};
use cashcontracts::{Address, AddressType};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use futures::Future;
use handlebars::Handlebars;
use panda_base::traits::*;

use crate::errors::*;
use dex_db::{models::DbPandaFull, panda_tools::*};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[derive(Serialize)]
struct PandaFrontEnd {
    pub token_id: String,
    pub owner_address: String,
    pub physique: PhysiqueTrait,
    pub pattern: PatternTrait,
    pub eye_color: EyeColorTrait,
    pub eye_shape: EyeShapeTrait,
    pub base_color: BaseColorTrait,
    pub highlight_color: HighlightColorTrait,
    pub accent_color: AccentColorTrait,
    pub wild_element: WildElementTrait,
}

impl From<DbPandaFull> for PandaFrontEnd {
    fn from(db_panda: DbPandaFull) -> PandaFrontEnd {
        let address = Address::from_bytes_prefix(
            "simpleledger",
            AddressType::P2PKH,
            (&db_panda.hash[..]).try_into().unwrap(),
        );
        PandaFrontEnd {
            token_id: hex::encode(db_panda.hash),
            owner_address: address.cash_addr().to_string(),
            physique: db_panda.physique,
            pattern: db_panda.pattern,
            eye_color: db_panda.eye_color,
            eye_shape: db_panda.eye_shape,
            base_color: db_panda.base_color,
            highlight_color: db_panda.highlight_color,
            accent_color: db_panda.accent_color,
            wild_element: db_panda.wild_element,
        }
    }
}

/// Get Panda by Address
fn breeders(
    hb: web::Data<Handlebars>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        // Get connection
        let conn: &PgConnection = &*pool
            .get()
            .map_err(|err| GetByAddressError::Connection(err.to_string()))?;

        // Grab breeders
        let breeders: Vec<String> = get_active_addresses(conn)
            .map_err(GetByAddressError::Diesel)?
            .into_iter()
            .filter_map(|x| x)
            .map(|pubkey_hash| {
                Address::from_bytes_prefix(
                    "simpleledger",
                    AddressType::P2PKH,
                    (&pubkey_hash[..]).try_into().unwrap(),
                )
                .cash_addr()
                .to_string()
            })
            .collect();

        // Convert to JSON
        let data = serde_json::to_value(breeders).map_err(GetByAddressError::Serde)?;

        // Render using handle bars
        Ok(hb
            .render("breeders", &data)
            .map_err(|err| GetByAddressError::Handlebars)?)
    })
    .then(
        // TODO: Fine grained error matching
        |res: Result<String, BlockingError<GetByAddressError>>| match res {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(_) => Ok(HttpResponse::NotFound().finish()),
        },
    )
}

/// Get Panda by Address
fn pandas_by_address(
    hb: web::Data<Handlebars>,
    pool: web::Data<Pool>,
    address: web::Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        // Get connection
        let conn: &PgConnection = &*pool
            .get()
            .map_err(|err| GetByAddressError::Connection(err.to_string()))?;

        // Decode token id
        let address =
            Address::from_cash_addr(address.to_string()).map_err(GetByAddressError::Address)?;

        // TODO: Validate it's an SLP address

        // Grab panda from DB
        let db_pandas =
            get_full_panda_by_addr(address.bytes(), &conn).map_err(GetByAddressError::Diesel)?;

        // Grab attributes
        let attributes: Vec<PandaFrontEnd> = db_pandas
            .into_iter()
            .map(|db_panda| PandaFrontEnd::from(db_panda))
            .collect();

        // Convert to JSON
        let data = serde_json::to_value(attributes).map_err(GetByAddressError::Serde)?;

        // Render using handle bars
        Ok(hb
            .render("pandas", &data)
            .map_err(|err| GetByAddressError::Handlebars)?)
    })
    .then(
        // TODO: Fine grained error matching
        |res: Result<String, BlockingError<GetByAddressError>>| match res {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(_) => Ok(HttpResponse::NotFound().finish()),
        },
    )
}

/// Get Panda by Token ID
fn panda_by_token_id(
    hb: web::Data<Handlebars>,
    pool: web::Data<Pool>,
    token_id: web::Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        // Get connection
        let conn: &PgConnection = &*pool
            .get()
            .map_err(|err| GetByTokenError::Connection(err.to_string()))?;

        // Decode token id
        let raw_token_id = hex::decode(&token_id.into_inner()).map_err(GetByTokenError::Hex)?;

        // Grab panda from DB
        let db_panda =
            get_full_panda_by_token_id(&raw_token_id, &conn).map_err(GetByTokenError::Diesel)?;

        // Convert to frontend panda
        let frontend_panda = PandaFrontEnd::from(db_panda);

        // Convert to JSON
        let data = serde_json::to_value(frontend_panda).map_err(GetByTokenError::Serde)?;

        // Render using handle bars
        Ok(hb
            .render("panda", &data)
            .map_err(|err| GetByTokenError::Handlebars)?)
    })
    .then(
        // TODO: Fine grained error matching
        |res: Result<String, BlockingError<GetByTokenError>>| match res {
            Ok(body) => Ok(HttpResponse::Ok().body(body)),
            Err(_) => Ok(HttpResponse::NotFound().finish()),
        },
    )
}

fn main() -> io::Result<()> {
    // Init handlebars
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // Init SQL connection
    let connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connection_str);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(index)
            .service(
                web::resource("/panda/{token_id}").route(web::get().to_async(panda_by_token_id)),
            )
            .service(
                web::resource("/pandas/{address}").route(web::get().to_async(pandas_by_address)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}

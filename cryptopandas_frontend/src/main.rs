#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_json;

pub mod errors;

use std::io;

use actix_web::{error::BlockingError, web, Error};
use actix_web::{App, HttpResponse, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use futures::Future;
use handlebars::Handlebars;
use panda_base::traits::*;

use crate::errors::GetByTokenError;
use dex_db::panda_tools::*;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

// #[get("/user/{user}/{data}")]
fn user(hb: web::Data<Handlebars>, info: web::Path<(String, String)>) -> HttpResponse {
    let data = json!({
        "user": info.0,
        "data": info.1
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
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
            get_panda_by_token_id(&raw_token_id, &conn).map_err(GetByTokenError::Diesel)?;

        // Grab attributes
        let attributes = db_panda
            .get_attributes()
            .map_err(|_| GetByTokenError::InvalidGene)?;

        // Convert to JSON
        let data = serde_json::to_value(attributes).map_err(GetByTokenError::Serde)?;

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
        // .service(user)
        // .service(panda_by_id)
    })
    .bind("127.0.0.1:8080")?
    .run()
}

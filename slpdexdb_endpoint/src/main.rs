mod actors;
mod msg;

use std::str::FromStr;
use std::net;
use std::sync::{Arc, Mutex};
use actix::prelude::*;
use diesel::prelude::*;

use actix_web::{middleware, web, App, HttpResponse, HttpRequest, HttpServer};
use actix_web_actors::ws;

use cashcontracts::{Address};
use slpdexdb_base::SLPDEXConfig;
use slpdexdb_db::Db;
use crate::actors::{TxActor, ResyncActor, PeersActor, WsActor};
use crate::msg::ConnectToPeer;

pub fn connect_db() -> Db {
    let connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let connection = PgConnection::establish(&connection_str).unwrap();
    Db::new(connection)
}

fn index(r: HttpRequest,
         stream: web::Payload,
         path: web::Path<(String,)>,
         tx: web::Data<Addr<TxActor>>) -> Result<HttpResponse, actix_web::Error> {
    let address_str = &path.0;
    eprintln!("connect to address {}", address_str);
    let address = Address::from_cash_addr(address_str.clone()).unwrap();  // TODO: handle error
    //Ok(HttpResponse::Ok().body("Hello"))
    ws::start(WsActor::new(address, tx.get_ref().clone()), &r, stream)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let port = std::env::var("PORT").unwrap_or("7501".to_string());
    actix::System::run(move || {
        let secret = hex::decode(std::env::var("SECRET").unwrap()).unwrap();
        let resync_addr = SyncArbiter::start(1, move || {
            ResyncActor::new(connect_db(), SLPDEXConfig::default(), secret.clone())
        });
        let db_addr = actors::DbActor::create().unwrap();
        let db_addr = slpdexdb_node::DbActor::start(slpdexdb_node::DbActor {
            add_header_query: db_addr.clone().recipient(),
            header_tip_query: db_addr.recipient(),
        });
        let tx_addr = TxActor::start_with(Arc::new(Mutex::new(connect_db())), SLPDEXConfig::default(), resync_addr);
        let peers_addr = PeersActor::start(PeersActor::new(tx_addr.clone(), db_addr));
        //let socket_addr = net::SocketAddr::from_str("147.135.131.25:8333").unwrap();
        //let socket_addr = net::SocketAddr::from_str("100.1.209.114:8333").unwrap();
        let socket_addr = net::SocketAddr::from_str("137.74.30.99:8333").unwrap();

        Arbiter::spawn(
            peers_addr.send(ConnectToPeer { socket_addr })
                .from_err()
                .and_then(|x| x)
                .map_err(|err| {eprintln!("{}", err);})
        );

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .data(tx_addr.clone())
                .service(
                    web::resource("/ws/{address}").route(web::get().to(index))
                )
        })
            .bind(format!("127.0.0.1:{}", port)).unwrap()
            .start();
    })?;
    Ok(())
}

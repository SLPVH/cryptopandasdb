#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};

use panda_base::traits::*;

use handlebars::Handlebars;

use std::io;

#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/user/{user}/{data}")]
fn user(hb: web::Data<Handlebars>, info: web::Path<(String, String)>) -> HttpResponse {
    let data = json!({
        "user": info.0,
        "data": info.1
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}

/// Example templating handler
#[get("/panda/{txid}")]
fn panda_by_id(hb: web::Data<Handlebars>, txid: web::Path<String>) -> HttpResponse {
    // TODO: Get from database
    let panda_attribute = PandaAttributes {
        physique: Physique::SmallFace,
        pattern: Pattern::Stripes,
        eye_color: EyeColor::Thundergrey,
        eye_shape: EyeShape::Caffeine,
        base_color: BaseColor::Harbourfog,
        highlight_color: HighlightColor::Lemonade,
        accent_color: AccentColor::Belleblue,
        wild_element: WildElement::ThirdEye,
        mouth: Mouth::Walrus,
    };
    let data = serde_json::to_value(panda_attribute).unwrap();
    let body = hb.render("panda", &data).unwrap();

    HttpResponse::Ok().body(body)
}

fn main() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .service(index)
            .service(user)
            .service(panda_by_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
}

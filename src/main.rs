use actix_web::{get, post, patch, App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Json, Path};
use validator::Validate;
use crate::models::{BuyPizzaRequest, UpdatePizzaURL };

mod config;
mod models;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available")
}

#[post("/buy_pizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        },
        Err(_) => HttpResponse::BadRequest().body("Pizza name invalid"),
    }
}

#[patch("/update_pizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid: String = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating the pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

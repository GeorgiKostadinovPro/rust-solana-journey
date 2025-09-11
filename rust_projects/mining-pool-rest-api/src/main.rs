#[macro_use]
extern crate actix_web;

// register modules in the crate
mod models;
mod controller;

// import modules from crate
use {
    actix_web::{middleware, App, HttpServer}
    std::{env, io}

    crate::models::*;
    crate::controllers::*;
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var();
    env_logger::init();

    // init the server
    HttpServer:new(|| {
        App::new().wrap(
            middleware::Logger::default()
        )
        .service(wallet_controller::get_all)
        .service(wallet_controller::get_by_id)
        .service(wallet_controller::create)
        .service(miner_controller::get_all)
        .service(miner_controller::get_by_id)
        .service(miner_controller::create)
    })
    .bind("0.0.0.0.9090")?
    .run()
    .await
}

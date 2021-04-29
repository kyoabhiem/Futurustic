#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
use crate::server::server;

mod api;
mod config;
mod database;
mod errors;
mod schema;
mod server;
mod validate;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}

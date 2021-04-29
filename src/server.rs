use crate::config::CONFIG;
use crate::database::add_pool;
use crate::{api, web};
use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut http_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(add_pool)
            .configure(api::routes)
            .configure(web::routes)
    });

    if CONFIG.worker != 0 {
        http_server = http_server.workers(CONFIG.worker)
    }

    http_server.bind(&CONFIG.server)?.run().await
}

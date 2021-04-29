use actix_web::web::{scope, ServiceConfig};

pub mod health;
pub mod users;

#[derive(Debug, Serialize)]
pub struct Respond<D, M> {
    pub data: D,
    pub meta: Option<M>,
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("api/v1")
            //Health
            .configure(health::routes::init_rotes)
            //Resource Users
            .configure(users::routes::init_rotes),
    );
}

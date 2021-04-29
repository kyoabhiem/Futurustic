pub mod landingpage;

use actix_web::web::{scope, ServiceConfig};
use actix_files::Files;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/")
            .configure(landingpage::routes::init_rotes)
            .service(Files::new("/", "public").disable_content_disposition()),
    );
}

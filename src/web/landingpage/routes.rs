use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "../src/web/landingpage/templates/index.html")]
struct Index;

#[get("")]
pub(crate) async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(Index.render().unwrap()))
}

pub fn init_rotes(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

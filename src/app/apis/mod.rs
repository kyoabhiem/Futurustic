use actix_web::{web, get, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello :D"
}

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::scope("v1")
        .service(index)
    );
}
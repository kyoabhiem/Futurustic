use actix_web::web;

mod landing_page;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(landing_page::index);
}
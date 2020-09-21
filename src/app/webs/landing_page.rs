use actix_web::{web, HttpResponse, Result, get};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
pub(crate) async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
            .render()
            .unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
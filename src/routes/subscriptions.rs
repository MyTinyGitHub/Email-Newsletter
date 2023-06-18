use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscription(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
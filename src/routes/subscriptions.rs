use actix_web::{HttpResponse, web};

pub async fn subscribe(_form: web::Form<SubscriberFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct SubscriberFormData {
    email: String,
    name: String
}

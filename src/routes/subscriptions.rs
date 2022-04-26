use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

pub async fn subscribe(
    form: web::Form<SubscriberFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
        println!("No can execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    }
    }
}

#[derive(serde::Deserialize)]
pub struct SubscriberFormData {
    email: String,
    name: String,
}

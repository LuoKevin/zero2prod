use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

pub async fn subscribe(
    form: web::Form<SubscriberFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    //For logs
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

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
        Ok(_) => {
            tracing::info!("request id {} - Successfully saved subscriber details.", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request id {} - No can execute query: {:?}",request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SubscriberFormData {
    email: String,
    name: String,
}

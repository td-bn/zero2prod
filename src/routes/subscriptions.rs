use actix_web::{web, HttpResponse,  Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct SubscriberData {
    name: String,
    email: String
}

pub async fn subscribe(
    form: web::Form<SubscriberData>,
    pool: web::Data<PgPool>
) -> impl Responder {
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
    .await {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            println!("Failed to execture query: {}", e);
            HttpResponse::InternalServerError()
        }
    }
}

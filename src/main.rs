use async_stripe::
use sqlx::postgres::PgPoolOptions;
use stripe::{Client as StripeClient};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required.");
    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY is required.");
    let stripe_webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").expect("STRIPE_WEBHOOK_SECRET is required.");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    let stripe_client = StripeClient::new(stripe_secret_key);
    
}

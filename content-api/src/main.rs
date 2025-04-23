use clap::Parser;
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use store::models::NewUser;

pub mod store;

#[derive(Parser)]
pub struct Opts {
    #[clap(long)]
    pub hostname: String,
    #[clap(long)]
    pub oidc_issuer: String,
    #[clap(long)]
    pub psql_uri: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    let opts = Opts::parse();
    let mut users: Vec<uuid::Uuid> = Vec::new();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let _client = reqwest::Client::new();
    let pool = Pool::builder()
            .build(AsyncDieselConnectionManager::<
                diesel_async::AsyncPgConnection,
            >::new(opts.psql_uri))
            .await?;

    let mut conn = pool.get().await?;

    for _ in 0..10 {
        users.push(uuid::Uuid::new_v4());
    }

    for public_id in users {
        diesel::insert_into(store::schema::users::table)
            .values(&NewUser {
                public_id
            })
            .execute(&mut conn)
            .await?;

        tracing::debug!({public_id = public_id.to_string()}, "[deck] inserting new user into database")
    }

    tracing::info!("[deck] content-api started and listening on {}", opts.hostname);

    Ok(())
}

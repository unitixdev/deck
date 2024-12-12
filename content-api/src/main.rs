use chrono::Utc;
use clap::Parser;
use diesel::dsl::insert_into;
use diesel_async::{pooled_connection::{bb8::Pool, AsyncDieselConnectionManager}, AsyncPgConnection, RunQueryDsl};

use serde_query::Deserialize;
use store::models::Set;

pub mod store;

#[derive(Deserialize, Debug)]
pub struct TcgSet {
    #[query(".id")]
    pub id: String,
    #[query(".name")]
    pub name: String,
    #[query(".cardCount.official")]
    pub count: i64
}

#[derive(Parser)]
pub struct Opts {
    #[clap(long, env = "HOSTNAME")]
    pub hostname: String,
    #[clap(long, env = "OIDC_ISSUER")]
    pub oidc_issuer: String,
    #[clap(long, env = "DATABASE_URL")]
    pub database_url: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    let opts = Opts::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client = reqwest::Client::new();
    let pool: Pool<AsyncPgConnection> = Pool::builder().build(AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(opts.database_url)).await?;

    let mut conn = pool.get().await?;

    let sets = client.get("https://api.tcgdex.net/v2/en/sets").send().await?.json::<Vec<TcgSet>>().await?;
    let sets = sets.into_iter().map(|set| {
        Set {
            id: set.id.clone(),
            name: set.name,
            count: set.count,
            url: format!("https://api.tcgdex.net/v2/en/sets/{}", set.id),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }).collect::<Vec<Set>>();

    insert_into(store::schema::card_sets::table)
        .values(&sets)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;

    Ok(())
}



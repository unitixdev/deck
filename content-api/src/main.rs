use std::{collections::HashMap, ops::Deref};

use clap::Parser;

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
async fn main() {
    let opts = Opts::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client = reqwest::Client::new();


    println!("Hello, world!");
}



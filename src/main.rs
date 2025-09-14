use axum::{extract::Query, routing::get, Router};
use std::{collections::HashMap, env};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .init();

    let app = Router::new().route("/api/hello", get(hello));
    let addr = format!(
        "0.0.0.0:{}",
        env::var("FUNCTIONS_CUSTOMHANDLER_PORT").map_or_else(|_| Ok(7071), |val| val.parse())?,
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello(Query(params): Query<HashMap<String, String>>) -> String {
    format!(
        "Hello, {}",
        params.get("name").unwrap_or(&"world".to_string())
    )
}

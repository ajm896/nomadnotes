mod db;
mod graphql;
mod models;

use crate::graphql::{graphql_handler, AppSchema, MutationRoot, QueryRoot};
use async_graphql::{EmptySubscription, Schema};

use axum::routing::options;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphql", options(|| async { "OK" }))
        .with_state(schema)
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("GraphQL API running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

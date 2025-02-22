mod cli;
mod db;
mod models;

//use cli::{Cli, Commands};

//use db::{delete_note, init_db, insert_note, print_table, retrive_note};
//use models::Note;
//use rusqlite::{Connection, Result};

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::post, Router};
use std::net::SocketAddr;
//xuse tokio::net::TcpListener;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }
}

use axum::extract::Extension;

async fn graphql_handler(
    Extension(schema): Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema)); // Attach schema as a shared state

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("GraphQL API running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

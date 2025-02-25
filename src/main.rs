mod db;
mod models;

use crate::db::{delete_note, get_all_notes, init_db, insert_note, retrive_note};
use crate::models::Note;

use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

struct QueryRoot;
struct MutationRoot;
type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[Object]
impl QueryRoot {
    async fn get_notes(&self, _ctx: &Context<'_>) -> async_graphql::Result<Vec<Note>> {
        let connection = init_db()?;
        let notes = get_all_notes(&connection)?;
        Ok(notes)
    }
}

#[Object]
impl MutationRoot {
    async fn add_note(
        &self,
        _ctx: &Context<'_>,
        title: String,
        content: String,
        tags: Vec<String>,
    ) -> async_graphql::Result<Note> {
        let connection = init_db()?;
        let note = Note {
            title: title.clone(),
            content: content.clone(),
            tags: tags.clone(),
            id: 0,
        };
        insert_note(&connection, note.clone())?;
        Ok(note)
    }

    async fn delete_note(
        &self,
        _ctx: &Context<'_>,
        title: String,
    ) -> async_graphql::Result<String> {
        let connection = init_db()?;
        delete_note(&connection, &title);
        Ok(title)
    }

    async fn edit_note(
        &self,
        _ctx: &Context<'_>,
        title: String,
        content: String,
    ) -> async_graphql::Result<Note> {
        let connection = init_db()?;
        let note = retrive_note(&connection, &title).unwrap();
        let new_note = Note {
            title: title.clone(),
            content: content.clone(),
            tags: note.tags.clone(),
            id: note.id,
        };
        delete_note(&connection, &title);
        insert_note(&connection, new_note.clone())?;
        Ok(new_note)
    }
}

async fn graphql_handler(schema: State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler).options(|| async { "OK" }))
        .with_state(schema)
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("GraphQL API running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

mod cli;
mod db;
mod models;

//use cli::{Cli, Commands};

use crate::db::{delete_note, get_all_notes, init_db, insert_note, retrive_note};
use crate::models::Note;

use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::post, Router};
use std::net::SocketAddr;

struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn get_notes(&self, _ctx: &Context<'_>) -> async_graphql::Result<Vec<Note>> {
        let connection = init_db()?;
        let notes = get_all_notes(&connection)?;
        Ok(notes)
    }
}

struct MutationRoot;
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

use axum::extract::Extension;

async fn graphql_handler(
    Extension(schema): Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

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

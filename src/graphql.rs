use crate::db::{delete_note, get_all_notes, init_db, insert_note, retrive_note};
use crate::models::Note;
pub use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
pub struct QueryRoot;
pub struct MutationRoot;
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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
    pub async fn add_note(
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

    pub async fn delete_note(
        &self,
        _ctx: &Context<'_>,
        title: String,
    ) -> async_graphql::Result<String> {
        let connection = init_db()?;
        delete_note(&connection, &title);
        Ok(title)
    }

    pub async fn edit_note(
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

pub async fn graphql_handler(schema: State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

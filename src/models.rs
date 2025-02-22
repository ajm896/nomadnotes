use async_graphql::SimpleObject;
use std::fmt::Display;

#[derive(Debug, SimpleObject)]
pub struct Note {
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}
impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nTags: {:?}\n\n{}",
            self.title, self.tags, self.content
        )
    }
}

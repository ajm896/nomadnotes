mod cli;
mod db;
mod models;

use cli::{Cli, Commands};

use db::{delete_note, init_db, insert_note, print_table, retrive_note};
use models::Note;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let connection: Connection = init_db()?;
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::New {
            title,
            tags,
            content,
        } => {
            let note = Note {
                title: title.clone(),
                tags: tags.clone().unwrap_or_default(),
                content: content.clone(),
            };
            insert_note(&connection, note)?;
        }
        Commands::List {} => {
            print_table(&connection)?;
        }
        Commands::View { title } => {
            match retrive_note(&connection, title) {
                Some(note) => println!("{}", note),
                None => println!("Note not Found"),
            };
        }
        Commands::Delete { title } => {
            delete_note(&connection, title);
        }
    }
    Ok(())
}

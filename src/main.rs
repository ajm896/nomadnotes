use std::fmt::Display;

use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    New {
        title: String,
        content: String,
        #[arg(short, long, num_args= 1.., value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    List,
    View {
        title: String,
    },
    Delete {
        title: String,
    },
}

#[derive(Debug)]
struct Note {
    title: String,
    tags: Vec<String>,
    content: String,
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

fn init_db() -> Result<Connection> {
    let connection = Connection::open("notes.db")?;
    connection.execute(
        "
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY,
                title TEXT UNIQUE NOT NULL,
                content TEXT NOT NULL,
                tags TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );",
        [],
    )?;
    Ok(connection)
}

fn insert_note(connection: &Connection, note: Note) -> Result<usize, rusqlite::Error> {
    connection.execute(
        "INSERT INTO notes (title, content, tags) VALUES (?1, ?2, ?3)",
        [note.title, note.content, note.tags.join(",")],
    )
}

fn print_table(connection: &Connection) -> Result<(), rusqlite::Error> {
    let sql = "SELECT * FROM notes;";

    let mut stmt = match connection.prepare(sql) {
        Ok(stmt) => stmt,
        Err(_) => panic!("Error while preparing statement"),
    };

    let notes = match stmt.query_map([], |row| parse_note(row)) {
        Ok(notes) => notes,
        Err(_) => panic!("Error while fetching notes"),
    };

    for note in notes {
        println!("{}", note?);
        println!("--------------------")
    }
    Ok(())
}

fn delete_note(connection: &Connection, title: &String) {
    match connection.execute("DELETE FROM notes WHERE title = ?1", [title]) {
        Ok(0) => println!("Note not found"),
        Ok(_) => println!("Note deleted"),
        Err(_) => println!("Error while deleting note"),
    };
}

fn parse_note(note: &rusqlite::Row) -> Result<Note> {
    Ok(Note {
        title: note.get(1)?,
        content: note.get(2)?,
        tags: note
            .get::<_, Option<String>>(3)?
            .unwrap_or_default()
            .split(",")
            .map(|s| s.to_string())
            .collect(),
    })
}

fn retrive_note(connection: &Connection, title: &String) -> Option<Note> {
    // Set up query
    let sql = "SELECT * FROM notes WHERE title = ?1;";
    let mut query = connection.prepare(sql).expect("SQL Error");

    // Execute query and map parse to Notes
    let mut notes = query.query_map([title], |n| parse_note(n)).ok()?;

    // Get the first note
    notes.next()?.ok()
}

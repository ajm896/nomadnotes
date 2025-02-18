use clap::{Parser, Subcommand};
use rusqlite::{Connection, MappedRows, Result};

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
fn main() -> Result<()> {
    let connection = init_db()?;
    let cli = Cli::parse();

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
            print_table(&connection);
        }
        Commands::View { title } => {}
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

fn print_table(connection: &Connection) {
    let mut stmt = connection.prepare("SELECT * FROM notes;").unwrap();
    let notes = stmt
        .query_map([], |row| {
            // Ok(Note {
            //     title: row.get(1)?,
            //     content: row.get(2)?,
            //     tags: row
            //         .get::<_, String>(3)?
            //         .split(",")
            //         .map(|s| s.to_string())
            //         .collect(),
            // })
            parse_note(row)
        })
        .unwrap();
    for note in notes {
        println!("{:?}", note.unwrap());
    }
}

fn delete_note(connection: &Connection, title: &String) {
    connection
        .execute("DELETE FROM notes WHERE title = ?1", [title])
        .unwrap();
}

fn parse_note(note: &rusqlite::Row) -> Result<Note> {
    Ok(Note {
        title: note.get(1).unwrap(),
        content: note.get(2).unwrap(),
        tags: note
            .get::<_, String>(3)
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .collect(),
    })
}

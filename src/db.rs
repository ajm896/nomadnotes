use super::models::Note;
use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
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

pub fn insert_note(connection: &Connection, note: Note) -> Result<usize, rusqlite::Error> {
    connection.execute(
        "INSERT INTO notes (title, content, tags) VALUES (?1, ?2, ?3)",
        [note.title, note.content, note.tags.join(",")],
    )
}

pub fn print_table(connection: &Connection) -> Result<(), rusqlite::Error> {
    let sql = "SELECT * FROM notes;";

    let mut stmt = match connection.prepare(sql) {
        Ok(stmt) => stmt,
        Err(_) => panic!("Error while preparing statement"),
    };

    let notes = match stmt.query_map([], parse_note) {
        Ok(notes) => notes,
        Err(_) => panic!("Error while fetching notes"),
    };

    for note in notes {
        println!("{}", note?);
        println!("--------------------")
    }
    Ok(())
}

pub fn delete_note(connection: &Connection, title: &String) {
    match connection.execute("DELETE FROM notes WHERE title = ?1", [title]) {
        Ok(0) => println!("Note not found"),
        Ok(_) => println!("Note deleted"),
        Err(_) => println!("Error while deleting note"),
    };
}

pub fn parse_note(note: &rusqlite::Row) -> Result<Note> {
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

pub fn retrive_note(connection: &Connection, title: &String) -> Option<Note> {
    // Set up query
    let sql = "SELECT * FROM notes WHERE title = ?1;";
    let mut query = connection.prepare(sql).expect("SQL Error");

    // Execute query and map parse to Notes
    let mut notes = query.query_map([title], parse_note).ok()?;

    // Get the first note
    notes.next()?.ok()
}
#[allow(dead_code)]
pub fn find_notes_by_tag(connection: &Connection, tag: &str) -> Result<Vec<Note>, rusqlite::Error> {
    let query = "SELECT * FROM notes WHERE tags LIKE ?1";
    let mut stmt = connection.prepare(query)?;
    let notes = stmt
        .query_map([format!("%{}%", tag)], |row| parse_note(row))?
        .filter_map(Result::ok)
        .collect();

    Ok(notes)
}

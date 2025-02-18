# 📖 NomadNotes – A Markdown-Powered Personal Knowledge Base
*A lightweight CLI tool for managing notes with SQLite persistence.*

---

## 🚀 Features
- ✅ Create, view, list, and delete notes.
- ✅ Store notes in SQLite with title, content, and tags.
- ✅ Search and filter notes by title.
- ✅ Version-controlled architecture (future goal).
- ✅ Portable and lightweight.

---

## 📦 Installation

### 1️⃣ Clone the Repository
```
git clone https://github.com/yourusername/nomadnotes.git  
cd nomadnotes
```

### 2️⃣ Install Dependencies
Ensure you have Rust installed. If not, install it via rustup.

```
cargo install --path .
```

---

## ⚡ Usage

### Create a New Note
```
nomadnotes new "My First Note" "This is the content" --tags rust,cli
```

### List All Notes
```
nomadnotes list
```

### View a Note
```
nomadnotes view "My First Note"
```

### Delete a Note
```
nomadnotes delete "My First Note"
```

---

## 🛠 Technologies Used
- **Rust** – Safe, fast, and memory-efficient.
- **SQLite (rusqlite)** – Lightweight, file-based database.
- **clap** – For parsing CLI commands.

---

## 📌 Roadmap
- ✅ Basic CRUD functionality.
- ⏳ Implement full-text search.
- ⏳ Add versioning for note edits.
- ⏳ Implement synchronization with Git or cloud storage.

---

## 👨‍💻 Contributing
Pull requests are welcome! Feel free to open an issue or discuss feature ideas.

---

## 📜 License
This project is open-source under the **MIT License**.

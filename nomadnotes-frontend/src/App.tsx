import { useQuery } from "@apollo/client";
import { GET_NOTES } from "./graphql/queries";
import NotesList from "./components/NotesList";
import { useState } from "react";
import NoteForm from "./components/NoteForm";
//import {BrowserRouter} from "react-router"



function App() {
  const { loading, error, data } = useQuery<{ getNotes: Note[] }>(GET_NOTES);
  const [editingNote, setEditingNote] = useState<Note | null>(null);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;
  
  return (
    <div>
      <h1>Nomad Notes</h1>
      {editingNote ? (
        <NoteForm
          initialTitle={editingNote.title}
          initialContent={editingNote.content}
          initialTags={editingNote.tags}
          isEditing
        />
      ) : (
        <NoteForm />
      )}
      <NotesList notes={data?.getNotes} editSelect={setEditingNote}/>
    </div>
  );
}

export default App;
import { useQuery } from "@apollo/client";
import { GET_NOTES } from "./graphql/queries";
import NotesList from "./components/NotesList";
import { useState } from "react";
import NoteForm from "./components/NoteForm";

function App() {
  const { loading, error, data } = useQuery<{ getNotes: Note[] }>(GET_NOTES);
  const [editingNote, setEditingNote] = useState<Note | null>(null);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;
  
  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-4xl font-bold text-center">Nomad Notes</h1>
        <div className="container mx-auto p-4">
          <NoteForm
            initialTitle={editingNote?.title || ""}
            initialContent={editingNote?.content || ""}
            initialTags={editingNote?.tags || []}
            isEditing={!!editingNote}
            onSave={() => setEditingNote(null)} // Reset editingNote after save
          />
          <NotesList notes={data?.getNotes} editSelect={setEditingNote} />
        </div>
    </div>
  );
}

export default App;
import { SetStateAction } from "react";
import "./NotesList.css"; // Import the CSS file

type CompProps = {
    notes?: Note[],
    editSelect: React.Dispatch<SetStateAction<Note | null>> 
}
export default function ListNotes(props: CompProps) {
    return (
      <div className="notes-grid">
        {props.notes?.map((note: Note) => (
          <div
            key={note.id}
            style={{ border: "1px solid #ddd", padding: "1em"}}
          >
            <h2>{note.title}</h2>
            <p>{note.content}</p>
            <p>Tags: {note.tags.join(", ")}</p>
            <button onClick={()=>props.editSelect(note)}>Edit</button>
            <button>Delete</button>
          </div>
        ))}
      </div>
    );
}
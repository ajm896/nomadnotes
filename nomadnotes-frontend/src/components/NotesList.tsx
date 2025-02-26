import { SetStateAction } from "react";
import { DELETE_NOTE } from "../graphql/mutations";
import { useMutation } from "@apollo/client";
import { GET_NOTES } from "../graphql/queries";
type CompProps = {
    notes?: Note[],
    editSelect: React.Dispatch<SetStateAction<Note | null>> 
}
export default function ListNotes(props: CompProps) {
  const [deleteNote] = useMutation(DELETE_NOTE, {
    refetchQueries: [{ query: GET_NOTES }],
  });
    return (
      <div className="grid grid-cols-3 gap-4 mt-4">
        {props.notes?.map((note: Note) => (
          <div
            key={note.id}
            className="border p-4 rounded shadow-md flex flex-col gap-2 bg-background"
          >
            <h2>{note.title}</h2>
            <p>Tags: {note.tags.join(", ")}</p>
            <span className="flex justify-end">
              <button
                className="bg-primary border-border justify-end rounded-2xl text-border"
                onClick={() => props.editSelect(note)}
              >
                Edit
              </button>
              <button
                onClick={() => {
                  const title = note.title;
                  deleteNote({ variables: { title } });
                }}
                className="shadow hover:bg-accent bg-error text-white"
              >
                Delete
              </button>
            </span>
          </div>
        ))}
      </div>
    );
}
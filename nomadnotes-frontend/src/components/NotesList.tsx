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
            className="border p-4 rounded-md shadow-md hover:shadow-lg"
          >
            <h2>{note.title}</h2>
            <p>Tags: {note.tags.join(", ")}</p>
            <span className="flex justify-end mt-4">
              <button
                className="btn rounded-full bg-blue-200 text-white hover:bg-blue-400 min-w-20"
                onClick={() => props.editSelect(note)}
              >
                Edit
              </button>
              <button onClick={() => {
                const title = note.title;
                  deleteNote({variables: {title}});
                }}
                      className="btn rounded-full shadow hover:bg-red-700 bg-red-500 text-white min-w-20">
                Delete
              </button>
            </span>
          </div>
        ))}
      </div>
    );
}
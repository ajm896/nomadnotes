import { useQuery } from "@apollo/client";
import { GET_NOTES } from "./graphql/queries";

type Note = {
  id: number;
  title: string;
  content: string;
  tags: string[];
};

function App() {
  const { loading, error, data } = useQuery<{ getNotes: Note[] }>(GET_NOTES);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <div>
      <h1>Nomad Notes</h1>
      {
      data?.getNotes.map((note) => (
        <div key={note.id} style={{ border: "1px solid #ddd", padding: "1em", margin: "1em" }}>
          <h2>{note.title}</h2>
          <p>{note.content}</p>
          <p>Tags: {note.tags.join(", ")}</p>
        </div>
      ))
      }
    </div>
  );
}

export default App;
import { useQuery } from "@apollo/client";
import { GET_NOTES } from "./graphql/queries";
import NotesList from "./components/NotesList"



function App() {
  const { loading, error, data } = useQuery<{ getNotes: Note[] }>(GET_NOTES);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <div>
      <h1>Nomad Notes</h1>
      <NotesList notes={data?.getNotes} />
    </div>
  );
}

export default App;
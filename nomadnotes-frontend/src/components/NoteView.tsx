type CompProps = {
  note?: Note;
};
export default function NotesList(props: CompProps) {
    const note: Note = props.note??{id: 0, title: "", content: "", tags: []};
    return (
      <div>
        <h2>{note.title}</h2>
        <h6>Tags: {note.tags.join(", ")}</h6>
        <p>{note.content}</p>
      </div>
    );
}

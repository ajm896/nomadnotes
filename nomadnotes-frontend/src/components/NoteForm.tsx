import { useState } from "react";
import { useMutation } from "@apollo/client";
import { ADD_NOTE, EDIT_NOTE } from "../graphql/mutations";
import { GET_NOTES } from "../graphql/queries";

type NoteFormProps = {
  initialTitle?: string;
  initialContent?: string;
  initialTags?: string[];
  isEditing?: boolean;
};

export default function NoteForm({
  initialTitle = "",
  initialContent = "",
  initialTags = [],
  isEditing = false,
}: NoteFormProps) {
  const [title, setTitle] = useState(initialTitle);
  const [content, setContent] = useState(initialContent);
  const [tags, setTags] = useState(initialTags.join(", "));

  const [addNote] = useMutation(ADD_NOTE, {
    refetchQueries: [{ query: GET_NOTES }], // Refresh notes list
  });

  const [editNote] = useMutation(EDIT_NOTE, {
    refetchQueries: [{ query: GET_NOTES }], // Refresh notes list
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const tagList = tags.split(",").map((tag) => tag.trim());

    if (isEditing) {
      await editNote({ variables: { title, content } });
    } else {
      await addNote({ variables: { title, content, tags: tagList } });
    }

    setTitle("");
    setContent("");
    setTags("");
  };

  return (
    <form
      onSubmit={handleSubmit}
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "1em",
        padding: "1em",
        border: "1px solid #ddd",
      }}
    >
      <label>
        Title:
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          required
        />
      </label>
      <label>
        Tags (comma-separated):
        <input
          type="text"
          value={tags}
          onChange={(e) => setTags(e.target.value)}
        />
      </label>
      <label>
        Content:
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          required
        />
      </label>
      <button type="submit">{isEditing ? "Update Note" : "Add Note"}</button>
    </form>
  );
}

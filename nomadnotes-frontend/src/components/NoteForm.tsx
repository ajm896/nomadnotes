import React, { useState, useEffect } from "react";
import { useMutation } from "@apollo/client";
import { ADD_NOTE, EDIT_NOTE } from "../graphql/mutations";
import { GET_NOTES } from "../graphql/queries";

interface NoteFormProps {
  initialTitle: string;
  initialContent: string;
  initialTags: string[];
  isEditing: boolean;
  onSave: () => void;
}

const NoteForm: React.FC<NoteFormProps> = ({ initialTitle, initialContent, initialTags, isEditing, onSave }) => {
  const [title, setTitle] = useState(initialTitle);
  const [content, setContent] = useState(initialContent);
  const [tags, setTags] = useState(initialTags);

  useEffect(() => {
    setTitle(initialTitle);
    setContent(initialContent);
    setTags(initialTags);
  
  }, [initialTitle, initialContent, initialTags]);

  const [addNote] = useMutation(ADD_NOTE, {
    refetchQueries: [{ query: GET_NOTES }], // Refresh notes list
  });

  const [editNote] = useMutation(EDIT_NOTE, {
    refetchQueries: [{ query: GET_NOTES }], // Refresh notes list
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const tagList = tags?.map((tag) => tag.trim());

    if (isEditing) {
      await editNote({ variables: { title, content } });
    } else {
      await addNote({ variables: { title, content, tags: tagList } });
    }

    setTitle("");
    setContent("");
    setTags([]);
    onSave();
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex flex-col gap-4 mt-4 border p-4 rounded-md shadow-md "
    >
      <label>
        Title:
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          required
          className="border p-2 rounded-md"
        />
      </label>
      <label>
        Tags (comma-separated):
        <input
          type="text"
          value={tags?.join(", ")}
          onChange={(e) => setTags(e.target.value.split(", "))}
          className="border p-2 rounded-md"
        />
      </label>
      <label>
        Content:
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          required
          className="border p-2 rounded-md w-full"
        />
      </label>
      <button className="bg-blue-500 text-white rounded-2xl" type="submit">{isEditing ? "Update Note" : "Add Note"}</button>
    </form>
  );
};

export default NoteForm;

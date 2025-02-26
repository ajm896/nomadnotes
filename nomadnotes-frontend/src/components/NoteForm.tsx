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
      className="border-border bg-background flex flex-col gap-1 rounded-md border p-4 shadow-md text-text"
    >
      <label>
        Title:
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          required
          className="border-border h-8 rounded-md border pl-2"
        />
      </label>
      <label>
        Tags (comma-separated):
        <input
          type="text"
          value={tags?.join(", ")}
          onChange={(e) => setTags(e.target.value.split(", "))}
          className="border-border h-8 rounded-md border pl-2"
        />
      </label>
      <label>
        Content:
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          required
          className="border-border h-100 w-full rounded-md border p-2"
        />
      </label>
      <button
        className="bg-primary border-border justify-end rounded-2xl text-border"
        type="submit"
      >
        {isEditing ? "Update Note" : "Add Note"}
      </button>
    </form>
  );
};

export default NoteForm;

import { gql } from "@apollo/client";

export const ADD_NOTE = gql`
  mutation AddNote($title: String!, $content: String!, $tags: [String!]!) {
    addNote(title: $title, content: $content, tags: $tags) {
      id
      title
      content
      tags
    }
  }
`;

export const EDIT_NOTE = gql`
  mutation EditNote($title: String!, $content: String!) {
    editNote(title: $title, content: $content) {
      id
      title
      content
      tags
    }
  }
`;

export const DELETE_NOTE = gql`
  mutation DeleteNote($title: String!) {
    deleteNote(title: $title)
  }
`;
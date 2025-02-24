import { gql } from "@apollo/client";

export const GET_NOTES = gql`
  query GetNotes {
    getNotes {
      id
      title
      content
      tags
    }
  }
`;

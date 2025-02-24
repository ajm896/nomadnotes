import { ApolloClient, InMemoryCache } from "@apollo/client";
const client = new ApolloClient({
  uri: "http://127.0.0.1:4000/graphql", // Connects to your Rust API
  cache: new InMemoryCache(), // Caches GraphQL responses
});

export default client;
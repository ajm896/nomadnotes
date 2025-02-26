import { ApolloClient, InMemoryCache } from "@apollo/client";
const client = new ApolloClient({
  uri: "https://nn.amorris.cc/graphql", // Connects to your Rust API
  cache: new InMemoryCache(), // Caches GraphQL responses
});

export default client;
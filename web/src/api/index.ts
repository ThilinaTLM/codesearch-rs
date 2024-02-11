import {HttpClient} from "@/lib/http-client.ts";
import {SearchResult} from "@/models";

const baseUrl = "http://localhost:3030";
const client = new HttpClient(baseUrl);

export type StandardResponse<T> = {
  results: T;
  error: string | null;
}


const api = {
  health: () => client.get<{status: string}>("/health"),
  search: (query: string) => client.get<StandardResponse<SearchResult[]>>("/search", {query}),
}

export default api;

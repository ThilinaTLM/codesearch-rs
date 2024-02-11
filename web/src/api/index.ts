import {HttpClient} from "@/lib/http-client.ts";
import {HealthResponse, ResultItem, SearchRequest, StandardResponse as SR} from "@/models";

const baseUrl = "http://localhost:3030";
const client = new HttpClient(baseUrl);




const api = {
  health: () => client.get<SR<HealthResponse>>("/api/health"),
  search: (body: SearchRequest) => client.post<SR<ResultItem[]>, SearchRequest>("/api/search", body),
}

export default api;

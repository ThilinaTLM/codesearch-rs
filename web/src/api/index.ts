import {HttpClient} from "@/lib/http-client.ts";
import {FileContentRequest, HealthResponse, ResultItem, SearchRequest, StandardResponse as SR} from "@/models";

const baseUrl = "http://localhost:3030";
const client = new HttpClient(baseUrl);


const api = {
    health: () => client.get<SR<HealthResponse>>("/api/health"),
    search: (body: SearchRequest) => client.post<SR<ResultItem[]>>("/api/search", body),
    fileContent: (form: FileContentRequest) => client.post<SR<string>>('/api/file/content', form),
    getRepos: () => client.get<SR<string[]>>("/api/repos"),
}

export default api;

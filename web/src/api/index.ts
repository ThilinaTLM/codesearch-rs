import {HttpClient} from "@/lib/http-client.ts";
import {FileContentRequest, HealthResponse, ResultItem, SearchRequest, StandardResponse as SR} from "@/models";
import {camelToSnakeObject} from "@/lib/convertor.ts";

const baseUrl = "http://localhost:3030";
const client = new HttpClient(baseUrl);


const api = {
  health: () => client.get<SR<HealthResponse>>("/api/health"),
  search: (body: SearchRequest) => client.post<SR<ResultItem[]>>("/api/search", body),
  fileContent: (repoName: string, path: string) => client.post<SR<string>>(
    '/api/file/content',
    camelToSnakeObject<FileContentRequest>({
      repoName,
      path
    })
  ),
}

export default api;

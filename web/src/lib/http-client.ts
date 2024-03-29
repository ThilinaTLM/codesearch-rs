export type HttpClientRequest = {
  method: "GET" | "POST" | "PUT" | "DELETE";
  path: string;
  data?: unknown;
  params?: Record<string, string>;
  headers?: Record<string, string>;
};


export class HttpClient {

  constructor(private url: string) {}

  async request<T>(request: HttpClientRequest): Promise<T> {

    // dist the url with query params
    const url = new URL(this.url);
    url.pathname = request.path;
    if (request.params) {
      Object.entries(request.params).forEach(([key, value]) => {
        url.searchParams.append(key, value);
      });
    }

    // set content type
    if (request.method === "POST" || request.method === "PUT") {
      request.headers = {
        "Content-Type": "application/json",
        ...request.headers,
      };
    }

    // make the request
    const response = await fetch(url.toString(), {
      method: request.method,
      headers: request.headers,
      body: request.data ? JSON.stringify(request.data) : undefined,
    });
    return response.json();

  }

  async get<T>(path: string, params?: Record<string, string>): Promise<T> {
    return this.request({method: "GET", path, params});
  }

  async post<T, D>(path: string, data: D): Promise<T> {
    return this.request({method: "POST", path, data});
  }

  async put<T, D>(path: string, data: D): Promise<T> {
    return this.request({method: "PUT", path, data});
  }

  async delete<T>(path: string): Promise<T> {
    return this.request({method: "DELETE", path});
  }

}

export type StandardResponse<T> = {
  data: T;
  error: string | null;
  time_taken: number;
}

export interface ResultItem {
  _score: number;
  repo_name: string;
  repo_path: string;
  repo_type: string;
  file_name: string;
  file_path: string;
  file_ext: string;
  file_size: number;
  file_last_updated: Date | string;
  file_language: string;
}

export interface HealthResponse {
  status: string;
}

export interface SearchRequest {
  query: string;
  limit?: number;
}

export interface FileContentRequest {
  repoName: string;
  path: string;
}

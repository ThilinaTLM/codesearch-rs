
export type StandardResponse<T> = {
  data: T;
  error: string | null;
  timeTaken: number;
}

export interface ResultItem {
  _score: number;
  repoName: string;
  repoPath: string;
  repoType: string;
  fileName: string;
  filePath: string;
  fileExt: string;
  fileSize: number;
  fileLastUpdated: Date | string;
}

export interface HealthResponse {
  status: string;
}

export interface SearchRequest {
  query: string;
  limit?: number;
  repos?: string[];
  fileTypes?: string[];
}

export interface FileContentRequest {
  repoName: string;
  path: string;
}

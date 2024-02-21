import {useEffect, useState} from "react";
import {ResultItem} from "@/models";
import api from "@/api/index.ts";

export function useSearchResults(query: string) {
  const [results, setResults] = useState<ResultItem[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (query) {
      setLoading(true);
      api.search({query, limit: 200}).then((res) => {
        setResults(res.data);
        setLoading(false);
      });
    }
  }, [query]);

  return {results, loading};
}

export function useFileContent(repoName: string, path: string) {
  const [content, setContent] = useState<string>("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setLoading(true);
    if (!repoName || !path) {
      setLoading(false);
      setContent("");
      return;
    }
    api.fileContent({repoName, path}).then((res) => {
      setContent(res.data);
      setLoading(false);
    });
  }, [repoName, path]);

  return {content, loading};
}
import {useEffect, useState} from "react";
import {SearchResult} from "@/models";
import api from "@/api/index.ts";

export function useSearchResults(query: string) {
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (query) {
      setLoading(true);
      api.search(query).then((res) => {
        setResults(res.results);
        setLoading(false);
      });
    }
  }, [query]);

  return {results, loading};
}
import {useEffect, useState} from "react";
import {ResultItem} from "@/models";
import api from "@/api/index.ts";

export function useSearchResults(query: string) {
  const [results, setResults] = useState<ResultItem[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (query) {
      setLoading(true);
      api.search({query, limit: 20}).then((res) => {
        setResults(res.data);
        setLoading(false);
      });
    }
  }, [query]);

  return {results, loading};
}
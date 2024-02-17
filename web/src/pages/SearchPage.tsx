import React, {useState} from 'react';
import {SearchBox} from "@/components/search/SearchBox.tsx";
import {SearchFileContent} from "@/components/search/SearchFileContent.tsx";
import {SearchFileList} from "@/components/search/SearchFileList.tsx";
import {useSearchResults} from "@/api/hooks.ts";
import {ResultItem} from "@/models";


export const SearchPage: React.FC = () => {
  const [query, setQuery] = useState('Math');
  const [selectedFile, setSelectedFile] = useState<ResultItem | null>(null);
  const {results} = useSearchResults(query)

  const handleSelectFile = React.useCallback((item: ResultItem) => {
    setSelectedFile(item)
  }, [])

  return (
    <div className="h-screen grid grid-rows-[auto,1fr]">
      <SearchBox onSearch={setQuery} />
      <div className="grid grid-cols-3 h-full overflow-hidden pb-3 px-5">
        <div className="col-span-1 overflow-auto p-4 h-full">
          <SearchFileList files={results} onSelect={handleSelectFile} />
        </div>
        <div className="col-span-2 overflow-auto p-4 h-full">
          <SearchFileContent item={selectedFile} />
        </div>
      </div>
    </div>
  );
};
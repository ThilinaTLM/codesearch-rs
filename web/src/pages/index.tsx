import React, {useState} from 'react';
import {SearchBox} from "@/components/SearchBox.tsx";
import {SearchFileContent} from "@/components/SearchFileContent.tsx";
import {SearchFileList} from "@/components/SearchFileList.tsx";
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
    <div className="h-screen grid grid-rows-[auto,1fr,30px]">
      <SearchBox onSearch={setQuery} />
      <div className="grid grid-cols-3 h-full overflow-hidden pb-3 px-5">
        <div className="col-span-1 overflow-auto p-4 h-full">
          <SearchFileList files={results} onSelect={handleSelectFile} />
        </div>
        <div className="col-span-2 overflow-auto p-4 h-full">
          <SearchFileContent item={selectedFile} />
        </div>
      </div>
      <div className="bg-black flex justify-center items-center">
        <p className="text-xs font-semibold text-white">
          Search Code - Powered by Rust + Tantivy
        </p>
      </div>
    </div>
  );
};
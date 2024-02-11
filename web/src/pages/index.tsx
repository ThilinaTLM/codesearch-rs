import React, {useState} from 'react';
import {SearchBox} from "@/components/SearchBox.tsx";
import {SearchFileContent} from "@/components/SearchFileContent.tsx";
import {SearchFileList} from "@/components/SearchFileList.tsx";
import {useSearchResults} from "@/api/hooks.ts";
import {SearchResult} from "@/models";



export const SearchPage: React.FC = () => {
  const [query, setQuery] = useState('');
  const [selectedFile, setSelectedFile] = useState<SearchResult | null>(null);
  const {results} = useSearchResults(query)

  const handleSelectFile = React.useCallback((item: SearchResult) => {
    setSelectedFile(item)
  }, [])

  return (
    <div>
      <SearchBox onSearch={setQuery}/>
      <div className="flex">
        <div className="w-1/3 p-4 overflow-auto">
          <SearchFileList files={results} onSelect={handleSelectFile}/>
        </div>
        <div className="w-2/3 p-4">
          <SearchFileContent content={selectedFile?.content || ''}/>
        </div>
      </div>
    </div>
  );
};
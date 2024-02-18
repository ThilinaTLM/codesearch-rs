import React, {useState} from 'react';
import {SearchBox} from "@/components/search/SearchBox.tsx";
import {SearchFileContent} from "@/components/search/SearchFileContent.tsx";
import {SearchFileList} from "@/components/search/SearchFileList.tsx";
import {useSearchResults} from "@/api/hooks.ts";
import {ResultItem} from "@/models";
import {
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable"


export const SearchPage: React.FC = () => {
  const [query, setQuery] = useState('Math');
  const [selectedFile, setSelectedFile] = useState<ResultItem | null>(null);
  const {results} = useSearchResults(query)

  const handleSelectFile = React.useCallback((item: ResultItem) => {
    setSelectedFile(item)
  }, [])

  React.useEffect(() => {
    setSelectedFile(results[0])
  }, [results])

  return (
    <div className="h-screen grid grid-rows-[auto,1fr,20px] justify-center">
      <SearchBox onSearch={setQuery} />
      <div style={{width: window.innerWidth - 100}}>
        <ResizablePanelGroup
          direction="horizontal"
          className="min-h-[200px] rounded-lg border"
        >
          <ResizablePanel defaultSize={50}>
            <SearchFileList
              files={results}
              onSelect={handleSelectFile}
              height={window.innerHeight - 130}
            />
          </ResizablePanel>

          <ResizablePanel defaultSize={50}>
            <SearchFileContent
              item={selectedFile}
              height={window.innerHeight - 130}
            />
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
};
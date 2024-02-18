import React, {useState} from 'react';
import {SearchBox} from "@/components/search/SearchBox.tsx";
import {FileContent} from "@/components/search/FileContent.tsx";
import {SearchFileList} from "@/components/search/SearchFileList.tsx";
import {useSearchResults} from "@/api/hooks.ts";
import {ResultItem} from "@/models";
import {ResizableHandle, ResizablePanel, ResizablePanelGroup,} from "@/components/ui/resizable"


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
      <SearchBox onSearch={setQuery}/>
      <div style={{width: window.innerWidth - 100}} className="pb-3">
        <ResizablePanelGroup
          direction="horizontal"
          className="min-h-[200px] rounded-lg border"
        >
          <ResizablePanel defaultSize={50}>
            <div className="p-3">
              <SearchFileList
                files={results}
                onSelect={handleSelectFile}
                height={window.innerHeight - 130}
              />
            </div>
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel defaultSize={50}>
            <div className="p-3">
              <FileContent
                item={selectedFile}
                height={window.innerHeight - 130}
              />
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  );
};
import React, {useState} from 'react';
import {SearchBox} from "@/components/search/SearchBox.tsx";
import {FileContent} from "@/components/search/FileContent.tsx";
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

    React.useEffect(() => {
        setSelectedFile(results[0])
    }, [results])

    return (
        <div className="h-screen pb-3 grid grid-rows-[70px,50px,1fr,20px] ">
            <SearchBox onSearch={setQuery}/>
            <div className=""></div>
            <div className="grid grid-cols-2 h-full">
                <div className="p-3">
                    <SearchFileList
                        files={results}
                        onSelect={handleSelectFile}
                    />
                </div>
                <div className="p-3">
                    <FileContent
                        item={selectedFile}
                        height={300}
                    />
                </div>
            </div>
        </div>
    );
};
import React from 'react';
import {Card} from '@/components/ui/card.tsx';
import {ResultItem} from "@/models";
import {cn} from "@/lib/utils.ts";

export type FileListProps = {
  files: ResultItem[];
  onSelect: (file: ResultItem) => void;
  height?: number;
}

function fileNameHighlighted(file: ResultItem) {
  return <>
    <span className="text-gray-500">{
      file.file_path.replace(file.file_name, '')
    }</span>
    <span className="font-medium">{file.file_name}</span>
  </>
}

export const SearchFileList: React.FC<FileListProps> = ({files, onSelect, height}) => {

  const [selected, setSelected] = React.useState<ResultItem | null>(null)

  React.useEffect(() => {
    setSelected(files[0])
  }, [files])

  React.useEffect(() => {
    if (selected) onSelect(selected)
  }, [selected, onSelect])

  if (files.length === 0) {
    return (
      <div className="text-center">
        <p>No files found</p>
      </div>
    );
  }

  return (
      <div className="overflow-y-scroll px-3" style={{height: height}}>
        <div>
          {files.map((file) => (
            <Card
              key={file.file_path}
              className={cn("cursor-pointer",
                selected === file && "border-l-2 border-primary",
                "hover:bg-gray-100",
              )}
              onClick={() => setSelected(file)}
            >
              <div className="flex justify-start overflow-hidden text-xs px-2 py-1">
                <div
                  className="whitespace-nowrap overflow-hidden text-ellipsis begin-truncate">{fileNameHighlighted(file)}</div>
              </div>
            </Card>
          ))}
        </div>
      </div>
  );
};

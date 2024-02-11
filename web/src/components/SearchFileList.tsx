import React from 'react';
import {Card, CardContent, CardDescription, CardTitle} from '@/components/ui/card';
import {ResultItem} from "@/models";

export type FileListProps = {
  files: ResultItem[];
  onSelect: (file: ResultItem) => void;
}

export const SearchFileList: React.FC<FileListProps> = ({files, onSelect}) => {

  if (files.length === 0) {
    return (
      <div className="text-center">
        <p>No files found</p>
      </div>
    );
  }

  return (
    <div className="overflow-auto space-y-2">
      {files.map((file) => (
        <Card key={file.file_path} className="cursor-pointer" onClick={() => onSelect(file)}>
          <CardContent className="p-3">
            <CardTitle className="text-sm truncate">{file.file_name}</CardTitle>
            <CardDescription className="text-xs truncate">Score: {file._score}</CardDescription>
            <CardDescription className="text-xs truncate">Path: {file.file_path}</CardDescription>
          </CardContent>
        </Card>
      ))}
    </div>
  );
};

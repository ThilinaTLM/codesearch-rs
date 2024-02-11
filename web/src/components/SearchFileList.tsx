import React from 'react';
import {Card, CardContent, CardDescription, CardTitle} from '@/components/ui/card';
import {SearchResult} from "@/models";

export type FileListProps = {
  files: SearchResult[];
  onSelect: (file: SearchResult) => void;
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
    <div className="space-y-4">
      {files.map((file) => (
        <Card key={file.path} className="cursor-pointer" onClick={() => onSelect(file)}>
          <CardContent className="p-3">
            <CardTitle className="text-lg font-semibold">{file.name}</CardTitle>
            <CardDescription>Score: {file.score}</CardDescription>
            <CardDescription>Path: {file.path}</CardDescription>
          </CardContent>
        </Card>
      ))}
    </div>
  );
};

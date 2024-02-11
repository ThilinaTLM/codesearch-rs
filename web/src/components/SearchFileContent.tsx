import React from 'react';

export type FileContentProps = {
  content: string;
}

export const SearchFileContent: React.FC<FileContentProps> = ({ content }) => {
  return (
    <div className="p-4 bg-gray-100 rounded-lg shadow">
      <pre className="whitespace-pre-wrap">
        <code>{content}</code>
      </pre>
    </div>
  );
};

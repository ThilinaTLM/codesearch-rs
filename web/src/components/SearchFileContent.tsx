import React from 'react';
import {ResultItem} from "@/models";
import SyntaxHighlighter from 'react-syntax-highlighter';
import { gruvboxDark } from 'react-syntax-highlighter/dist/esm/styles/hljs';


export type FileContentProps = {
  item?: ResultItem | null;
}

export const SearchFileContent: React.FC<FileContentProps> = ({item}) => {

  const language = React.useMemo(() => {
    if (!item) {
      return 'plaintext';
    }
    const ext = item.file_name.split('.').pop();
    if (ext === 'ts' || ext === 'tsx') {
      return 'typescript';
    }
    if (ext === 'js' || ext === 'jsx') {
      return 'javascript';
    }
    if (ext === 'py') {
      return 'python';
    }
    if (ext === 'java') {
      return 'java';
    }
    return 'plaintext';
  }, [item])


  if (!item) {
    return (
      <div className="p-4 bg-gray-100 rounded-lg shadow h-full">
        <pre className="whitespace-pre-wrap text-center text-xs">
          <code>No file selected</code>
        </pre>
      </div>
    );
  }

  return (
    <div className="h-full overflow-auto rounded">
      <pre className="text-xs">
        <SyntaxHighlighter
          language={language}
          style={{
            ...gruvboxDark,
            hljs: {
              ...gruvboxDark.hljs,
              padding: '20px',
              borderRadius: '0.5em',
            }
        }}
        >
          {item.file_content}
        </SyntaxHighlighter>
      </pre>
    </div>
  );
};

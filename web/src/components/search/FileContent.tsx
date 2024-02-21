import React from 'react';
import {ResultItem} from "@/models";
import {useFileContent} from "@/api/hooks.ts";
import CodeBlock from "@/components/CodeBlock.tsx";


export type FileContentProps = {
  item?: ResultItem | null;
  height?: number;
}

export const FileContent: React.FC<FileContentProps> = ({item, height}) => {

  const {content, loading} = useFileContent(item?.repoName || '', item?.filePath || '')

  if (loading) {
    return <div className="text-xs flex items-center justify-center h-full">
      <div>Loading....</div>
    </div>
  }

  if (!content) {
    return <div className="text-xs flex items-center justify-center h-full">
      <div>No Content</div>
    </div>
  }

  return (
    <div style={{height: `${height}px`}} className="overflow-auto">
      <CodeBlock
        extension={item?.fileExt || ''}
        value={content}
      />
    </div>
  );
};

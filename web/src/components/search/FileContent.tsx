import React from 'react';
import {ResultItem} from "@/models";
import {useFileContent} from "@/api/hooks.ts";
import CodeBlock from "@/components/CodeBlock.tsx";


export type FileContentProps = {
  item?: ResultItem | null;
  height?: number;
}

export const FileContent: React.FC<FileContentProps> = ({item, height}) => {

  const {content, loading} = useFileContent(item?.repo_name || '', item?.file_path || '')

  if (loading) {
    return <div>Loading...</div>
  }

  if (!content) {
    return <div>No content</div>
  }

  return (
    <div style={{height: `${height}px`}} className="overflow-auto">
      <CodeBlock
        extension={item?.file_ext || ''}
        value={content}
      />
    </div>
  );
};

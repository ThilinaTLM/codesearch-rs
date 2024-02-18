import React from 'react';
import {ResultItem} from "@/models";
import {useFileContent} from "@/api/hooks.ts";
import {ScrollArea} from "@/components/ui/scroll-area.tsx";


export type FileContentProps = {
  item?: ResultItem | null;
  height?: number;
}

export const SearchFileContent: React.FC<FileContentProps> = ({item, height}) => {

  const {content, loading} = useFileContent(item?.repo_name || '', item?.file_path || '')

  if (loading) {
    return <div>Loading...</div>
  }

  if (!content) {
    return <div>No content</div>
  }

  return (
      <ScrollArea style={{height: `${height}px`}} className="p-3 ">
      <pre>
        <code>
          {content}
        </code>
      </pre>
      </ScrollArea>
  );
};

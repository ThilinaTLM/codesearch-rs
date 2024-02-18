import React, { useEffect } from 'react';
import hljs from 'highlight.js';
import 'highlight.js/styles/github.css';
import {extToLang} from "@/lib/extToLang.ts"; // Example with GitHub theme

interface CodeBlockProps {
  extension: string;
  value: string;
}

const CodeBlock: React.FC<CodeBlockProps> = ({ extension, value }) => {

  useEffect(() => {
    hljs.highlightAll();
  }, []);

  const language = extToLang(extension);

  return (
    <pre style={{fontFamily: 'Ubuntu Mono'}} className="text-xs">
      <code className={`language-${language}`}>
        {value}
      </code>
    </pre>
  );
};

export default CodeBlock;

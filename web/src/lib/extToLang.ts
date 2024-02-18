
const extensionToLanguageMap: { [key: string]: string } = {
  js: 'javascript',
  ts: 'typescript',
  jsx: 'javascript',
  tsx: 'typescript',
  py: 'python',
  sh: 'bash',
  bash: 'bash',
  md: 'markdown',
  json: 'json',
  html: 'xml', // Highlight.js uses 'xml' for HTML
  css: 'css',
  scss: 'scss',
  sass: 'sass',
  less: 'less',
  yml: 'yaml',
  yaml: 'yaml',
  cpp: 'cpp',
  c: 'c',
  h: 'c', // Header files in C can also be highlighted as C code
  java: 'java',
  class: 'java', // Compiled Java files won't be highlighted but listed for completeness
  kt: 'kotlin',
  kts: 'kotlin',
  swift: 'swift',
  rb: 'ruby',
  php: 'php',
  cs: 'csharp',
  fs: 'fsharp',
  go: 'go',
  rs: 'rust',
  pl: 'perl',
  r: 'r',
  lua: 'lua',
  sql: 'sql',
  ps1: 'powershell',
  bat: 'dos',
  xml: 'xml',
  ini: 'ini',
  toml: 'toml',
  dockerfile: 'dockerfile',
};

export function extToLang(ext: string): string {
  return extensionToLanguageMap[ext] || 'plaintext';
}

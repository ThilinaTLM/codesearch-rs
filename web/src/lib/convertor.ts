
export function camelToSnake(s: string): string {
  return s.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
}

export function camelToSnakeObject<T>(obj: Record<string, unknown>): T {
  const newObj: Record<string, unknown> = {};
  for (const key in obj) {
    newObj[camelToSnake(key)] = obj[key];
  }
  return newObj as T;
}
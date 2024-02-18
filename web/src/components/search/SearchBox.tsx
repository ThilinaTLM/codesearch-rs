import React, { useState } from 'react';
import { Input } from '@/components/ui/input.tsx';
import { Button } from '@/components/ui/button.tsx';

interface SearchBoxProps {
  onSearch: (query: string) => void;
}

function useDebounce(value: string, delay: number) {
  const [debouncedValue, setDebouncedValue] = useState(value);

  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
}

export const SearchBox: React.FC<SearchBoxProps> = ({ onSearch }) => {

  const [query, setQuery] = useState('');
  const debouncedQuery = useDebounce(query, 500);

  const handleSearch = () => {
    onSearch(query);
  };

  React.useEffect(() => {
    onSearch(debouncedQuery);
  }, [debouncedQuery, onSearch]);

  return (
    <div className="flex justify-center my-8">
      <div className="flex w-full max-w-xl items-center space-x-2">
        <Input
          className="flex-1"
          type="text"
          placeholder="Search..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
        />
        <Button onClick={handleSearch} type="button">Search</Button>
      </div>
    </div>
  );
};

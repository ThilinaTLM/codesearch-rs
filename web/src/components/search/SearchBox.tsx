import React, { useState } from 'react';
import { Input } from '@/components/ui/input.tsx';
import { Button } from '@/components/ui/button.tsx';

interface SearchBoxProps {
  onSearch: (query: string) => void;
}

export const SearchBox: React.FC<SearchBoxProps> = ({ onSearch }) => {
  const [query, setQuery] = useState('');

  const handleSearch = () => {
    onSearch(query);
  };

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setQuery(e.target.value);
    onSearch(e.target.value);
  }

  return (
    <div className="flex justify-center my-8">
      <div className="flex w-full max-w-xl items-center space-x-2">
        <Input
          className="flex-1"
          type="text"
          placeholder="Search..."
          value={query}
          onChange={onChange}
          onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
        />
        <Button onClick={handleSearch} type="button">Search</Button>
      </div>
    </div>
  );
};

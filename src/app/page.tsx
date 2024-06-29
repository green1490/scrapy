"use client"

import List from '@/components/list';
import { useEffect, useState } from 'react';

export default function Home() {
  const [companies, setCompanies] = useState()
  useEffect(() => {
    const companies = fetch('')
  })
  
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div>
        <List
          careerSites={[
            
          ]}
        />
      </div>
    </main>
  );
}

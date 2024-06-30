import List from '@/components/list';
import { CareerSite } from '@/model/careerSite';
import { ApiError } from 'next/dist/server/api-utils';

async function listCompaniesData():Promise<CareerSite[]> {
  const companiesURL = process.env.NEXT_PUBLIC_COMPANY_URL
  if (companiesURL === undefined) {
    console.log(companiesURL)
    throw new ApiError(400, 'The url doesnt have any value!')
  }
  const companies = await fetch(companiesURL)
  if (!companies.ok) {
    throw new Error('Failed to fetch data!')
  }
  return await companies.json()
}

export default async function Home() {
  return (
    <main className="">
      <div>
        <List
          careerSites={await listCompaniesData()}
        />
      </div>
    </main>
  );
}

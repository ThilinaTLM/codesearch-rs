import { createLazyFileRoute } from '@tanstack/react-router'
import {SearchPage} from "@/pages/SearchPage.tsx";



export const Route = createLazyFileRoute('/')({
  component: () => <SearchPage />
})
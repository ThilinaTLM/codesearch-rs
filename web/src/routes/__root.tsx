import { createRootRoute, Outlet } from '@tanstack/react-router'
import Footer from "@/components/layout/Footer.tsx";

export const Route = createRootRoute({
  component: () => (
    <>
      <Outlet />
      <Footer/>
    </>
  ),
})
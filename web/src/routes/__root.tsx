import { createRootRoute, Outlet } from '@tanstack/react-router'
import Footer from "@/components/layout/Footer.tsx";

export const Route = createRootRoute({
  component: () => (
    <div className="h-screen grid grid-rows-[98vh,1fr] overflow-hidden">
      <Outlet />
      <Footer />
    </div>
  ),
})
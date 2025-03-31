import { createFileRoute, Link, Outlet } from "@tanstack/react-router";
import { Clapperboard } from "lucide-react";

export const Route = createFileRoute("/_auth")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main className="min-h-screen bg-gray-100 flex flex-col items-center py-20">
      <div className="w-72 rounded-lg bg-white border border-gray-200 p-4 text-center flex flex-col items-center">
        <div className="absolute -mt-9">
          <Link to="/">
            <Clapperboard
              className="relative bg-emerald-500 p-2 text-white rounded-md hover:bg-emerald-600"
              size={40}
            />
          </Link>
        </div>
        <Outlet />
      </div>
    </main>
  );
}

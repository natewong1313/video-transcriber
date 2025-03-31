import { createFileRoute, Link } from "@tanstack/react-router";
import { Clapperboard } from "lucide-react";

export const Route = createFileRoute("/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main className="min-h-screen flex flex-col bg-gray-100">
      <nav className="flex justify-between items-center py-4 bg-white px-8 border-b border-gray-200">
        <div className="flex items-center justify-center space-x-2">
          <Clapperboard
            className="bg-emerald-500 p-2 text-white rounded-md"
            size={40}
          />
          <h1>nate's video transcriber</h1>
        </div>
        <Link to="/login" className="hover:underline">
          please login
        </Link>
      </nav>
    </main>
  );
}

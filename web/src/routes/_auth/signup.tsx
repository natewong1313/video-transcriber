import { createFileRoute, Link } from "@tanstack/react-router";

export const Route = createFileRoute("/_auth/signup")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <h1 className="mt-4 font-medium text-lg">sign up!</h1>
      <form className="text-sm mt-4 w-full">
        <input
          className="w-full rounded-md border border-gray-200 px-2 py-2 focus:ring-1 focus:ring-emerald-200 focus:outline-none"
          placeholder="email"
          type="email"
        />
        <input
          className="mt-4 w-full rounded-md border border-gray-200 px-2 py-2 focus:ring-1 focus:ring-emerald-200 focus:outline-none"
          placeholder="password"
          type="password"
        />
        <input
          className="mt-4 w-full rounded-md border border-gray-200 px-2 py-2 focus:ring-1 focus:ring-emerald-200 focus:outline-none"
          placeholder="confirm password"
          type="password"
        />
        <button className="bg-emerald-500 hover:bg-emerald-600 focus:bg-emerald-700 text-white rounded-md w-full py-2 mt-4">
          continue
        </button>
      </form>
      <p className="text-sm mt-4 text-gray-600">
        have an account already?{" "}
        <Link to="/login" className="underline">
          sign in here
        </Link>
      </p>
    </>
  );
}

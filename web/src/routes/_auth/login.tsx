import { Updater, useForm } from "@tanstack/react-form";
import { createFileRoute, Link } from "@tanstack/react-router";

export const Route = createFileRoute("/_auth/login")({
  component: RouteComponent,
});

function Input({
  name,
  value,
  onBlur,
  onChange,
}: {
  name: string;
  value: string;
  onBlur: () => void;
  onChange: (updater: Updater<string>) => void;
}) {
  return (
    <input
      className="w-full mt-2 rounded-md border border-gray-200 px-2 py-2 focus:ring-1 focus:ring-emerald-200 focus:outline-none"
      placeholder={name}
      type={name}
      value={value}
      onBlur={onBlur}
      onChange={(e) => onChange(e.target.value)}
    />
  );
}

function RouteComponent() {
  const form = useForm({
    defaultValues: {
      email: "",
      password: "",
    },
    onSubmit: async ({ value }) => {
      console.log(value);
    },
  });
  return (
    <>
      <h1 className="mt-4 font-medium text-lg">login</h1>
      <form
        className="text-sm mt-4 w-full"
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
      >
        <form.Field
          name="email"
          children={(field) => (
            <Input
              name={field.name}
              value={field.state.value}
              onBlur={field.handleBlur}
              onChange={field.handleChange}
            />
          )}
        ></form.Field>
        <form.Field
          name="password"
          children={(field) => (
            <Input
              name={field.name}
              value={field.state.value}
              onBlur={field.handleBlur}
              onChange={field.handleChange}
            />
          )}
        ></form.Field>

        <form.Subscribe
          selector={(state) => [state.canSubmit, state.isSubmitting]}
          children={([canSubmit, isSubmitting]) => (
            <button
              type="submit"
              className="bg-emerald-500 hover:bg-emerald-600 focus:bg-emerald-700 text-white rounded-md w-full py-2 mt-4"
              disabled={!canSubmit}
            >
              {isSubmitting ? "..." : "continue"}
            </button>
          )}
        />
      </form>
      <p className="text-sm mt-4 text-gray-600">
        need an account?{" "}
        <Link to="/signup" className="underline">
          create one here
        </Link>
      </p>
    </>
  );
}

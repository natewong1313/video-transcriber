import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { Clapperboard } from "lucide-react";
import { getAuthenticatedUser } from "../lib/auth";
import { useFilePicker } from "use-file-picker";
import { FileContent } from "use-file-picker/types";

export const Route = createFileRoute("/")({
  component: RouteComponent,
  loader: () => getAuthenticatedUser(),
});

function RouteComponent() {
  const user = Route.useLoaderData();
  const navigate = useNavigate();

  const { openFilePicker, filesContent, loading } = useFilePicker({
    accept: ".mp4",
  });

  const onOpenFilePicker = () => {
    if (user) {
      openFilePicker();
    } else {
      navigate({ to: "/login" });
    }
  };
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
        {user ? (
          <Link to="/login" className="hover:underline">
            dashboard
          </Link>
        ) : (
          <Link to="/login" className="hover:underline">
            please login
          </Link>
        )}
      </nav>
      <div className="flex py-16 items-center flex-col">
        <h1 className="text-5xl font-bold">transcribe video to text</h1>
        <p className="mt-2">
          simply upload your video below and get the full transcript
        </p>
        {filesContent.length == 0 ? (
          <UploadFileComponent
            openFilePicker={onOpenFilePicker}
            loading={loading}
          />
        ) : (
          <ViewFileComponent
            openFilePicker={onOpenFilePicker}
            filesContent={filesContent}
          />
        )}
      </div>
    </main>
  );
}

function UploadFileComponent({
  openFilePicker,
  loading,
}: {
  openFilePicker: () => void;
  loading: boolean;
}) {
  return (
    <div className="mt-8 bg-white h-56 w-[30rem] rounded-md flex items-center justify-center flex-col">
      <h2 className="text-lg font-medium">upload your file</h2>
      <p className="mt-2">
        click the choose file button or drag and drop files below
      </p>
      <button
        onClick={openFilePicker}
        className="bg-emerald-500 hover:bg-emerald-600 focus:bg-emerald-700 text-white rounded-md py-2 px-4 mt-4"
        disabled={loading}
      >
        {loading ? "..." : "choose file"}
      </button>
    </div>
  );
}

function ViewFileComponent({
  openFilePicker,
  filesContent,
}: {
  openFilePicker: () => void;
  filesContent: FileContent<string>[];
}) {
  const loading = false;
  return (
    <div className="mt-8 bg-white h-56 w-[30rem] p-6 rounded-md flex items-center justify-center flex-col">
      <h2 className="text-lg font-medium">all set!</h2>
      <p className="my-2">
        review your file below and press transcribe when ready
      </p>
      {filesContent.map((file) => (
        <div className="text-xs w-full bg-gray-100 rounded-md px-2 py-4 flex items-center">
          <p className="truncate mr-2">{file.name}</p>
          <button
            onClick={openFilePicker}
            className="ml-auto rounded-md py-2 px-3 ring ring-gray-400 hover:bg-gray-200"
          >
            change
          </button>
        </div>
      ))}
      <button
        className="bg-emerald-500 hover:bg-emerald-600 focus:bg-emerald-700 text-white rounded-md py-2 px-4 mt-4"
        disabled={loading}
      >
        {loading ? "..." : "transcribe"}
      </button>
    </div>
  );
}

import { Clapperboard } from "lucide-react";

function App() {
  return (
    <main className="min-h-screen flex flex-col bg-gray-100">
      <nav className="flex justify-between items-center py-6 bg-white px-8 border-b border-gray-200">
        <div className="flex items-center justify-center space-x-2">
          <Clapperboard
            className="bg-emerald-500 p-2 text-white rounded-md"
            size={40}
          />
          <h1>nate's video transcriber</h1>
        </div>
        <a
          href="https://www.github.com/natewong1313/video-transcriber"
          target="_blank"
          className="hover:underline"
        >
          please login
        </a>
      </nav>
    </main>
  );
}

export default App;

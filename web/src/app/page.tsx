import FileUploader from "@/components/file-uploader";

export default function Home() {
  return (
    <main>
      <nav className="w-full py-4 px-8 border-b border-neutral-700 text-white flex justify-between items-center">
        <div>
          video transcriber{" "}
          <span className="text-sm text-neutral-400">(beta)</span>
        </div>
        <a
          href="https://www.github.com/natewong1313/video-transcriber"
          target="_blank"
          className="bg-violet-500 hover:bg-violet-600 active:scale-90 text-white font-medium px-4 py-1.5 text-sm rounded-sm transition-all duration-100"
        >
          view source
        </a>
      </nav>
      <div className="flex items-center justify-center flex-col font-plex py-16">
        <h1 className="text-white font-semibold text-4xl">Video Transcriber</h1>
        <p className="text-white mt-2">
          Transcribe a video or audio recording with the tool below.
        </p>
        <FileUploader />
      </div>
    </main>
  );
}

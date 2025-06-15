import cn from "@/lib/cn";
import {
  DownloadSimpleIcon,
  FileIcon,
  TrashSimpleIcon,
} from "@phosphor-icons/react";

function humanFileSize(size: number) {
  const i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024));
  return (
    +(size / Math.pow(1024, i)).toFixed(2) * 1 +
    " " +
    ["B", "kB", "MB", "GB", "TB"][i]
  );
}

export default function FileCard({
  file,
  onDelete,
  status,
  onDownload,
}: {
  file: File;
  onDelete: () => void;
  status: "notStarted" | "inProgress" | "done";
  onDownload: () => void;
}) {
  return (
    <div
      className={cn(
        "border border-neutral-700 rounded-sm p-3 flex items-start bg-neutral-800/50 justify-between",
        status == "inProgress" && "opacity-60"
      )}
    >
      <div className="flex items-center space-x-2 overflow-hidden flex-auto mr-2">
        <div className="border border-neutral-700 p-3 rounded-sm text-white">
          <FileIcon size={24} />
        </div>
        <div>
          <p className="text-white">{file.name}</p>
          <p className="text-neutral-500 text-sm">
            {humanFileSize(file.size)}
            {status === "done" && (
              <span>
                {" "}
                • <span className="text-emerald-400 text-md">completed!</span>
              </span>
            )}
          </p>
        </div>
      </div>

      {status === "notStarted" && (
        <button
          className="text-neutral-500 p-1 hover:bg-neutral-800 rounded-sm hover:text-neutral-400 transition-all duration-100"
          onClick={onDelete}
        >
          <TrashSimpleIcon size={24} />
        </button>
      )}
      {status === "done" && (
        <button
          className="text-white p-1 hover:bg-neutral-800 rounded-sm hover:text-neutral-400 transition-all duration-100"
          onClick={onDownload}
        >
          <DownloadSimpleIcon size={24} />
        </button>
      )}
    </div>
  );
}

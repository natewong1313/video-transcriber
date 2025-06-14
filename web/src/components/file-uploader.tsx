"use client";

import cn from "@/lib/cn";
import { FolderIcon, FolderOpenIcon } from "@phosphor-icons/react";
import { useState } from "react";
import { FileWithPath, useDropzone } from "react-dropzone";
import { DotLottieReact } from "@lottiefiles/dotlottie-react";
import { useMutation } from "@tanstack/react-query";
import FileCard from "./file-card";

export default function FileUploader() {
  const [uploadedFiles, setUploadedFiles] = useState<FileWithPath[]>([]);
  const {
    getRootProps,
    getInputProps,
    isDragActive,
    open: openDialog,
  } = useDropzone({
    onDrop: (newFiles) => {
      const filePaths = new Set<string>();
      const filteredFiles = [...uploadedFiles, ...newFiles].filter(
        (file: FileWithPath) => {
          if (!file.path || filePaths.has(file.path)) {
            return false;
          }
          filePaths.add(file.path);
          return true;
        }
      );
      setUploadedFiles(filteredFiles);
    },
    noClick: true,
    noKeyboard: true,
    multiple: true,
    accept: {
      "audio/mpeg": [".mp3"],
      "audio/wav": [".wav"],
      "video/mp4": [".mp4"],
      "video/mpeg": [".mpeg"],
    },
  });

  const transcribeMutation = useMutation({
    mutationFn: async (files: FileWithPath[]) => {
      const formData = new FormData();
      files.forEach((file) => formData.append("files", file));
      const response = await fetch("/api/transcribe", {
        method: "POST",
        body: formData,
      });
      const reader = response.body
        ?.pipeThrough(new TextDecoderStream())
        .getReader();
      if (!reader) {
        throw new Error("failed to read");
      }
      while (true) {
        const { value, done } = await reader.read();
        if (done) {
          console.log("done");
          return;
        }
        console.log(value);
      }
    },
  });

  const DisplayedFolderIcon = isDragActive ? FolderOpenIcon : FolderIcon;
  return (
    <div className="max-w-[36rem]">
      <div
        className={cn(
          "mt-8 px-36 py-12  border border-neutral-700 rounded-sm flex flex-col justify-center items-center transition-all duration-100",
          isDragActive
            ? "border-neutral-500 bg-neutral-800/70"
            : "bg-neutral-800/50"
        )}
        {...getRootProps()}
      >
        <DisplayedFolderIcon
          size="64"
          weight="duotone"
          className="text-violet-500"
        />
        <h1 className="text-white font-bold text-3xl mt-2">
          Drag & drop <span className="text-violet-500">video</span>
          <br /> or <span className="text-violet-500">audio</span> files here
        </h1>
        <p className="text-white mt-3 font-medium">
          or <input {...getInputProps()} />
          <button
            type="button"
            onClick={openDialog}
            className="text-violet-500 underline underline-offset-2 cursor-pointer"
          >
            browse files
          </button>{" "}
          on your computer
        </p>
      </div>
      {/* display uploaded files  */}
      <div className="flex flex-col mt-4 space-y-2">
        {uploadedFiles.map((file) => (
          <FileCard
            key={file.name}
            file={file}
            onDelete={() =>
              setUploadedFiles(uploadedFiles.filter((f) => f.path != file.path))
            }
          />
        ))}
      </div>
      {uploadedFiles.length > 0 && (
        <button
          className="bg-violet-500 hover:bg-violet-600 active:scale-90 text-white font-medium text-sm rounded-sm transition-all duration-100 w-full mt-4 flex items-center justify-center"
          onClick={() => transcribeMutation.mutate(uploadedFiles)}
        >
          {/* transcribe files */}
          {transcribeMutation.isPending ? (
            <DotLottieReact
              src="/audio-animation.lottie"
              loop
              autoplay
              className="h-10"
            />
          ) : (
            <p className="h-10 flex items-center">transcribe files</p>
          )}
        </button>
      )}
    </div>
  );
}

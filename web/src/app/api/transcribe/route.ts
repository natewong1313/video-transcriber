import pool from "@/lib/db-pool";
import { PutObjectCommand } from "@aws-sdk/client-s3";
import s3client from "@/lib/s3-client";
import { v4 as uuidv4 } from "uuid";

export type TaskResult = {
  id: string;
  fileName: string;
  transcript: string;
};

export async function POST(request: Request) {
  const formData = await request.formData();
  const files = formData.getAll("files") as File[];

  const stream = new ReadableStream({
    async start(controller) {
      await Promise.all(files.map((file) => runTask(file, controller)));
      controller.close();
    },
  });
  return new Response(stream, {
    headers: {
      Connection: "keep-alive",
      "Content-Encoding": "none",
      "Cache-Control": "no-cache, no-transform",
      "Content-Type": "text/event-stream; charset=utf-8",
    },
  });
}

async function runTask(
  file: File,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  controller: ReadableStreamDefaultController<any>
) {
  const taskId = uuidv4();
  const fileUrl = await uploadFile(file, taskId);
  const client = await pool.connect();
  await client.query(
    "INSERT INTO TASKS(id, url, status) VALUES ($1, $2, 'notStarted')",
    [taskId, fileUrl]
  );
  await client.query(`LISTEN task_${taskId.replaceAll("-", "_")}_done`);

  return new Promise((resolve, reject) => {
    client.on("notification", async (notif) => {
      if (!notif.payload) {
        return reject("missing payload");
      }
      // strip out status and url, we dont wanna expose the s3 url
      const { id, transcript, url } = JSON.parse(notif.payload);
      const fileName = url.substring(url.lastIndexOf("/") + 1);
      controller.enqueue(
        new TextEncoder().encode(JSON.stringify({ id, transcript, fileName }))
      );
      resolve(":)");
    });
    client.on("error", reject);
  });
}

// upload to r2 & get the url
async function uploadFile(file: File, taskId: string) {
  const fileBody = await file.arrayBuffer();
  const fileKey = `${taskId}/${file.name}`;

  const command = new PutObjectCommand({
    Bucket: "video-transcriber",
    Key: fileKey,
    // @ts-expect-error weird aws
    Body: fileBody,
    ContentType: file.type,
  });

  await s3client.send(command);
  return `https://videos3.nate-wong.com/${fileKey}`;
}

import { S3Client } from "@aws-sdk/client-s3";
import invariant from "tiny-invariant";
import { loadEnvConfig } from "@next/env";

loadEnvConfig(process.cwd());

invariant(process.env.R2_ACCOUNT_ID, "missing R2_ACCOUNT_ID");
invariant(process.env.R2_ACCESS_KEY_ID, "missing R2_ACCESS_KEY_ID");
invariant(process.env.R2_SECRET_ACCESS_KEY, "missing R2_SECRET_ACCESS_KEY");

const s3client = new S3Client({
  region: "auto",
  endpoint: `https://${process.env.R2_ACCOUNT_ID}.r2.cloudflarestorage.com`,
  credentials: {
    accessKeyId: process.env.R2_ACCESS_KEY_ID,
    secretAccessKey: process.env.R2_SECRET_ACCESS_KEY,
  },
});

export default s3client;

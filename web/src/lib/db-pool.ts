import invariant from "tiny-invariant";
import { URL } from "url";
import { Pool } from "pg";

invariant(process.env.DATABASE_URL, "missing DATABASE_URL");
const parsedUrl = new URL(process.env.DATABASE_URL);

const pool = new Pool({
  user: parsedUrl.username,
  password: parsedUrl.password,
  host: parsedUrl.hostname || "",
  port: parseInt(parsedUrl.port || ""),
  database: parsedUrl.pathname?.split("/")[1],
  ssl: process.env.NODE_ENV == "production",
});
export default pool;

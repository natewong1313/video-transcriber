import { betterAuth } from "better-auth";
import pool from "./db-pool";

export const auth = betterAuth({
  database: pool,
  emailAndPassword: {
    enabled: true,
  },
});

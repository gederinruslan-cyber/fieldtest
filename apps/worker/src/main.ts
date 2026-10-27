import { Worker } from "bullmq";

const redis = new URL(process.env.REDIS_URL ?? "redis://127.0.0.1:26379");

// No job types yet: imports, showcases and purges arrive with their features.
const worker = new Worker(
  "jobs",
  async (job) => {
    throw new Error(`unknown job: ${job.name}`);
  },
  { connection: { host: redis.hostname, port: Number(redis.port) } },
);

process.on("SIGTERM", () => void worker.close());

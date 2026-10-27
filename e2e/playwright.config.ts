import { defineConfig, devices } from "@playwright/test";

// Production builds on the host against the Compose services (verify:full).
export default defineConfig({
  testDir: "tests",
  forbidOnly: true,
  use: { baseURL: "http://127.0.0.1:3200" },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
  webServer: [
    { command: "pnpm --filter @fieldtest/web start", url: "http://127.0.0.1:3200" },
    { command: "pnpm --filter @fieldtest/api start", url: "http://127.0.0.1:3201/health" },
  ],
});

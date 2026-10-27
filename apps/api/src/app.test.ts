import { expect, test } from "vitest";
import { buildApp } from "./app.js";

test("GET /health answers ok", async () => {
  const res = await buildApp().inject({ method: "GET", url: "/health" });
  expect(res.statusCode).toBe(200);
  expect(res.json()).toEqual({ status: "ok" });
});

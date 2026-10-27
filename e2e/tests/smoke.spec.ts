import { expect, test } from "@playwright/test";

test("the home page loads and the api answers", async ({ page, request }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Fieldtest");
  await expect(page.getByText("study tool for use away from the table")).toBeVisible();

  const health = await request.get("http://127.0.0.1:3201/health");
  expect(await health.json()).toEqual({ status: "ok" });
});

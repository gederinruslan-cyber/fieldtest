import { expect, test } from "@playwright/test";

test("exact equity for two hands, an estimate with its margin for two ranges", async ({ page }) => {
  await page.goto("/tools/equity");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Equity calculator");

  await page.getByRole("textbox", { name: "Player 1" }).fill("AhAs");
  await page.getByRole("textbox", { name: "Player 2" }).fill("KdKc");
  await page.getByRole("button", { name: "Calculate" }).click();
  const exact = page.getByRole("table", { name: /^Exact: 1,712,304 deals/ });
  const cells = (table: typeof exact, name: string) =>
    table.getByRole("row", { name }).getByRole("cell");
  await expect(cells(exact, "Player 1")).toHaveText(["81.26%", "81.06%", "0.38%", "AhAs"]);

  await page.getByRole("textbox", { name: "Player 1" }).fill("AA");
  await page.getByRole("textbox", { name: "Player 2" }).fill("KK");
  await page.getByRole("button", { name: "Calculate" }).click();
  await page.getByRole("textbox", { name: "Player 1" }).fill("AA;KK");
  await page.getByRole("button", { name: "Calculate" }).click();
  await expect(page.getByText("A range isn't recognised")).toBeVisible();
  await page.getByRole("textbox", { name: "Player 1" }).fill("AA");
  await page.getByRole("button", { name: "Calculate" }).click();
  const estimate = page.getByRole("table", { name: /^Estimate: Monte Carlo over 200,000/ });
  await expect(cells(estimate, "Player 1").first()).toHaveText(/^\d+\.\d\d%±0\.\d\d pp$/);
});

import { expect, test } from "@playwright/test";

test("exact ICM for three stacks and two payouts", async ({ page }) => {
  await page.goto("/tools/icm");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("ICM calculator");

  for (const [player, stack] of [
    ["Player 1", "50"],
    ["Player 2", "30"],
    ["Player 3", "20"],
  ] as const) {
    await page.getByRole("textbox", { name: player }).fill(stack);
  }
  await page.getByRole("textbox", { name: "Payouts" }).fill("70, 30");
  await page.getByRole("button", { name: "Calculate" }).click();
  const table = page.getByRole("table", { name: /^Exact: Malmuth-Harville/ });
  const equity = (name: string) => table.getByRole("row", { name }).getByRole("cell").first();
  await expect(equity("Player 1")).toHaveText("45.18");
  await expect(equity("Player 2")).toHaveText("32.25");
  await expect(equity("Player 3")).toHaveText("22.57");
});

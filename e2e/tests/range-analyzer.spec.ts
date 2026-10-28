import { expect, test } from "@playwright/test";

test("analyse a range on the flop and carry the kept hands to the turn", async ({ page }) => {
  await page.goto("/tools/range-analyzer");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Range analyzer");

  const range = page.getByRole("textbox", { name: "Range" });
  await range.fill("AA,AK");
  await page.getByRole("button", { name: "Apply range" }).click();
  await expect(page.getByRole("button", { name: /^AA,/ })).toHaveAttribute("aria-pressed", "true");

  for (const card of ["Ace of hearts", "Seven of clubs", "Two of diamonds"]) {
    await page.getByRole("button", { name: card }).click();
  }
  const breakdown = page.getByRole("table", { name: /Exact/ });
  await expect(breakdown).toContainText("15 live combos");
  const cells = (name: RegExp) => breakdown.getByRole("row", { name }).getByRole("cell");
  await expect(cells(/^Set/)).toHaveText(["3", "20.0%", ""]);
  await expect(cells(/^Top pair/)).toHaveText(["12", "80.0%", ""]);

  await breakdown.getByRole("checkbox", { name: "Keep Set" }).check();
  await page.getByRole("button", { name: "Keep selected and go to the turn" }).click();
  await expect(range).toHaveValue("AdAc,AsAc,AsAd");
  await expect(page.getByText("Pick the turn card")).toBeVisible();
});

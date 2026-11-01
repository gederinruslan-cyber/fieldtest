import { expect, test } from "@playwright/test";

test("heads-up push/fold Nash at 10 big blinds", async ({ page }) => {
  await page.goto("/tools/push-fold");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Push/fold Nash calculator");
  await page.getByRole("button", { name: "Calculate" }).click();
  await expect(
    page.getByText(
      /^Nash equilibrium: neither player can gain (even|more than) [0-9.]+% of the prize pool/,
    ),
  ).toBeVisible();
  await expect(page.getByRole("table", { name: /^Small blind pushes: / })).toHaveAccessibleName(
    "Small blind pushes: 58.5% of hands",
  );
  await expect(page.getByRole("table", { name: /^Big blind calls: / })).toHaveAccessibleName(
    "Big blind calls: 37.3% of hands",
  );
});

/** The one place a share becomes text: a percentage to `digits` decimals (one by default),
 * rounded once, here. */
export function formatShare(share: number, digits = 1): string {
  return `${(share * 100).toFixed(digits)}%`;
}

/** A standard error as a margin in percentage points, to two decimals. */
export function formatMargin(error: number): string {
  return `±${(error * 100).toFixed(2)} pp`;
}

/** Weighted combo counts: whole numbers as they are, fractional weights to two decimals. */
export function formatCombos(weight: number): string {
  return Number.isInteger(weight) ? String(weight) : weight.toFixed(2);
}

/** The one place a share becomes text: a percentage to one decimal, rounded once, here. */
export function formatShare(share: number): string {
  return `${(share * 100).toFixed(1)}%`;
}

/** Weighted combo counts: whole numbers as they are, fractional weights to two decimals. */
export function formatCombos(weight: number): string {
  return Number.isInteger(weight) ? String(weight) : weight.toFixed(2);
}

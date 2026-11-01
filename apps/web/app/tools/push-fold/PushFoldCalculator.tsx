"use client";

import { createBrowserEnginePool, type EnginePool } from "@fieldtest/engine-wasm";
import { type FormEvent, useEffect, useRef, useState } from "react";
import { formatShare } from "../../../lib/format";
import styles from "../calculator.module.css";
import grid from "./pushfold.module.css";

const RANKS = "AKQJT98765432";
const ERRORS: Record<string, string> = {
  InvalidStacks:
    "Every stack needs whole chips with some left after its blind and ante, and there are at most 20 players.",
  InvalidBlinds: "The small blind must be above 0 and below the big blind; the ante can be 0.",
  InvalidPayouts:
    "Payouts are numbers from first place down: at least one, none negative, no more than there are players.",
};
const FIELDS = [
  ["small", "Small blind stack"],
  ["big", "Big blind stack"],
  ["sb", "Small blind"],
  ["bb", "Big blind"],
  ["ante", "Ante"],
] as const;
type Field = (typeof FIELDS)[number][0];

type Result = { exploitability: number; push: number; call: number; grids: number[][] };

/** Grid cell label: pairs on the diagonal, suited above it, offsuit below. */
function cellLabel(row: number, col: number): string {
  const [high, low] = [RANKS[Math.min(row, col)], RANKS[Math.max(row, col)]];
  return row === col ? `${high}${low}` : `${high}${low}${row < col ? "s" : "o"}`;
}

function RangeGrid({ title, share, weights }: { title: string; share: number; weights: number[] }) {
  const mixed = weights.flatMap((w, i) =>
    w > 0 && w < 1 ? [`${cellLabel(Math.floor(i / 13), i % 13)} ${formatShare(w, 0)}`] : [],
  );
  return (
    <div className={grid.range}>
      <h3>{title}</h3>
      <p className={grid.share}>{formatShare(share)}</p>
      <table className={grid.grid} aria-label={`${title}: ${formatShare(share)} of hands`}>
        <tbody>
          {RANKS.split("").map((rank, row) => (
            <tr key={rank}>
              {RANKS.split("").map((_, col) => {
                const label = cellLabel(row, col);
                const w = weights[row * 13 + col] ?? 0;
                const state = w >= 1 ? "full" : w > 0 ? "partial" : "none";
                return (
                  <td key={label} data-weight={state}>
                    {label}
                    <span className={grid.hidden}>
                      {state === "full"
                        ? ", always"
                        : state === "partial"
                          ? `, ${formatShare(w, 0)}`
                          : ", never"}
                    </span>
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
      {mixed.length > 0 && <p className={grid.mixed}>Mixed: {mixed.join(", ")}</p>}
    </div>
  );
}

export function PushFoldCalculator() {
  const pool = useRef<EnginePool | null>(null);
  const [values, setValues] = useState<Record<Field, string>>({
    small: "1000",
    big: "1000",
    sb: "50",
    bb: "100",
    ante: "0",
  });
  const [others, setOthers] = useState("");
  const [payouts, setPayouts] = useState("100");
  const [result, setResult] = useState<Result | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    pool.current = createBrowserEnginePool();
  }, []);

  async function calculate(event: FormEvent) {
    event.preventDefault();
    if (!pool.current) return;
    const v = Object.fromEntries(FIELDS.map(([k]) => [k, values[k].trim()])) as Record<
      Field,
      string
    >;
    const single = FIELDS.find(([k]) => /[\s,/]/.test(v[k]) || v[k] === "");
    const code = single
      ? single[0] === "small" || single[0] === "big"
        ? "InvalidStacks"
        : "InvalidBlinds"
      : null;
    const r = code
      ? { ok: false as const, code }
      : await pool.current.call(
          "pushFold",
          [v.small, v.big, others].join(" "),
          [v.sb, v.bb, v.ante].join(" "),
          payouts,
        );
    if (!r.ok || r.value.length !== 3 + 2 * 169) {
      const failed = r.ok ? "InvalidStacks" : r.code;
      setError(ERRORS[failed] ?? `The engine refused this input (${failed}).`);
      setResult(null);
      return;
    }
    const out = [...r.value];
    setError(null);
    setResult({
      exploitability: out[0] ?? 0,
      push: out[1] ?? 0,
      call: out[2] ?? 0,
      grids: [out.slice(3, 172), out.slice(172)],
    });
  }

  // Below the shown precision, say so rather than print 0.000%.
  const epsilon =
    result && result.exploitability < 0.00001
      ? "even 0.001%"
      : `more than ${formatShare(result?.exploitability ?? 0, 3)}`;

  return (
    <div className={styles.workspace}>
      <form className={styles.panel} onSubmit={calculate} aria-labelledby="spot-heading">
        <h2 id="spot-heading">Spot</h2>
        {FIELDS.map(([key, label]) => (
          <div key={key} className={styles.field}>
            <label htmlFor={key}>{label}</label>
            <input
              id={key}
              className={styles.input}
              value={values[key]}
              onChange={(e) => setValues({ ...values, [key]: e.target.value })}
              inputMode="numeric"
              autoComplete="off"
            />
          </div>
        ))}
        <div className={styles.field}>
          <label htmlFor="others">Other stacks (folded)</label>
          <input
            id="others"
            className={styles.input}
            value={others}
            onChange={(e) => setOthers(e.target.value)}
            placeholder="Optional, as in 3000, 1500"
            autoComplete="off"
          />
        </div>
        <div className={styles.field}>
          <label htmlFor="payouts">Payouts</label>
          <input
            id="payouts"
            className={styles.input}
            value={payouts}
            onChange={(e) => setPayouts(e.target.value)}
            placeholder="First place down, as in 50, 30, 20"
            autoComplete="off"
          />
        </div>
        <button type="submit" className={styles.primary}>
          Calculate
        </button>
        {error && (
          <p className={styles.error} role="alert">
            {error}
          </p>
        )}
      </form>

      {result && (
        <section className={styles.panel} aria-labelledby="result-heading">
          <h2 id="result-heading">Nash ranges</h2>
          <p id="method" className={styles.method}>
            Nash equilibrium: neither player can gain {epsilon} of the prize pool by changing a hand
            (exact heads-up equities)
          </p>
          <div className={grid.ranges}>
            <RangeGrid
              title="Small blind pushes"
              share={result.push}
              weights={result.grids[0] ?? []}
            />
            <RangeGrid
              title="Big blind calls"
              share={result.call}
              weights={result.grids[1] ?? []}
            />
          </div>
        </section>
      )}
    </div>
  );
}

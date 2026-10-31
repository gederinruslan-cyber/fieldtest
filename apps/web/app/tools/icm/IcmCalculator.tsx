"use client";

import { createBrowserEnginePool, type EnginePool } from "@fieldtest/engine-wasm";
import { type FormEvent, useEffect, useRef, useState } from "react";
import { formatAmount, formatChips, formatShare } from "../../../lib/format";
import styles from "../calculator.module.css";

const MAX_PLAYERS = 20;
const ERRORS: Record<string, string> = {
  InvalidStacks: "Every player needs a whole number of chips above 0.",
  InvalidPayouts:
    "Payouts are numbers from first place down: at least one, none negative, no more than there are players.",
};

type Result = { seat: number; stack: number; equity: number; prize: number; chips: number }[];

export function IcmCalculator() {
  const pool = useRef<EnginePool | null>(null);
  const [stacks, setStacks] = useState(["5000", "3000", "2000"]);
  const [payouts, setPayouts] = useState("50, 30, 20");
  const [result, setResult] = useState<Result | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    pool.current = createBrowserEnginePool();
  }, []);

  async function calculate(event: FormEvent) {
    event.preventDefault();
    if (!pool.current) return;
    const typed = stacks.map((s) => s.trim());
    // Each row is one stack; a separator inside a row would add a player.
    const r = typed.some((s) => /[\s,/]/.test(s))
      ? { ok: false as const, code: "InvalidStacks" }
      : await pool.current.call("icm", typed.join(" "), payouts);
    if (!r.ok || r.value.length !== 3 * typed.length) {
      const code = r.ok ? "InvalidStacks" : r.code;
      setError(ERRORS[code] ?? `The engine refused this input (${code}).`);
      setResult(null);
      return;
    }
    const v = r.value;
    setError(null);
    setResult(
      typed.map((stack, i) => ({
        seat: i + 1,
        stack: Number(stack),
        equity: v[3 * i] ?? 0,
        prize: v[3 * i + 1] ?? 0,
        chips: v[3 * i + 2] ?? 0,
      })),
    );
  }

  return (
    <div className={styles.workspace}>
      <form className={styles.panel} onSubmit={calculate} aria-labelledby="stacks-heading">
        <h2 id="stacks-heading">Stacks and payouts</h2>
        {stacks.map((value, i) => (
          // Rows are only added or removed at the end, so the position is a stable key.
          // biome-ignore lint/suspicious/noArrayIndexKey: see above
          <div key={i} className={styles.field}>
            <label htmlFor={`stack-${i}`}>Player {i + 1}</label>
            <input
              id={`stack-${i}`}
              className={styles.input}
              value={value}
              onChange={(e) => setStacks(stacks.map((s, j) => (j === i ? e.target.value : s)))}
              inputMode="numeric"
              autoComplete="off"
            />
          </div>
        ))}
        <div className={styles.actions}>
          <button
            type="button"
            className={styles.secondary}
            disabled={stacks.length >= MAX_PLAYERS}
            onClick={() => setStacks([...stacks, ""])}
          >
            Add player
          </button>
          <button
            type="button"
            className={styles.secondary}
            disabled={stacks.length <= 2}
            onClick={() => setStacks(stacks.slice(0, -1))}
          >
            Remove player {stacks.length}
          </button>
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
          <h2 id="result-heading">ICM equity</h2>
          <p id="method" className={styles.method}>
            Exact: Malmuth-Harville ICM, every finishing order counted
          </p>
          <div className={styles.scroll}>
            <table className={styles.table} aria-labelledby="method">
              <thead>
                <tr>
                  <th scope="col">Player</th>
                  <th scope="col">Equity</th>
                  <th scope="col">Prize share</th>
                  <th scope="col">Chips</th>
                  <th scope="col">Chip share</th>
                </tr>
              </thead>
              <tbody>
                {result.map((p) => (
                  <tr key={p.seat}>
                    <th scope="row">Player {p.seat}</th>
                    <td>{formatAmount(p.equity)}</td>
                    <td>{formatShare(p.prize, 2)}</td>
                    <td>{formatChips(p.stack)}</td>
                    <td>{formatShare(p.chips, 2)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      )}
    </div>
  );
}

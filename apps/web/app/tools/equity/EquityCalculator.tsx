"use client";

import { createBrowserEnginePool, type EnginePool } from "@fieldtest/engine-wasm";
import { type FormEvent, useEffect, useRef, useState } from "react";
import { formatMargin, formatShare } from "../../../lib/format";
import styles from "./equity.module.css";

/** Monte Carlo runs this many deals with a fixed seed, so the same spot shows the same numbers. */
const SAMPLES = 200_000;
const SEED = 1;
const ERRORS: Record<string, string> = {
  InvalidToken: "A range isn't recognised. Write a hand like AhKh or a range like 22+, ATs+, KQo.",
  InvalidWeight: "Weights must be above 0 and at most 1, as in AKs:0.5.",
  InvalidCard: "A board card isn't recognised. Write cards like Ah 7c 2d.",
  InvalidBoard: "The board is 3, 4 or 5 different cards, or empty for preflop.",
  InvalidPlayers: "Every player needs a hand or a range.",
  NoValidDeal: "No deal was found in which these hands and the board share no card.",
};

type Result = {
  exact: boolean;
  count: number;
  players: {
    seat: number;
    range: string;
    win: number;
    tie: number;
    equity: number;
    error: number;
  }[];
};

export function EquityCalculator() {
  const pool = useRef<EnginePool | null>(null);
  const [players, setPlayers] = useState(["", ""]);
  const [board, setBoard] = useState("");
  const [result, setResult] = useState<Result | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    pool.current = createBrowserEnginePool();
  }, []);

  async function calculate(event: FormEvent) {
    event.preventDefault();
    if (!pool.current) return;
    const ranges = players.map((p) => p.trim());
    // The engine separates players with `;`, so one inside a row would add a player.
    if (ranges.some((range) => range.includes(";"))) {
      setError(ERRORS.InvalidToken ?? null);
      setResult(null);
      return;
    }
    setBusy(true);
    const r = await pool.current.call("equity", ranges.join(";"), board, SAMPLES, SEED);
    setBusy(false);
    if (!r.ok || r.value.length !== 2 + 4 * ranges.length) {
      const code = r.ok ? "InvalidPlayers" : r.code;
      setError(ERRORS[code] ?? `The engine refused this input (${code}).`);
      setResult(null);
      return;
    }
    const v = r.value;
    setError(null);
    setResult({
      exact: v[0] === 1,
      count: v[1] ?? 0,
      players: ranges.map((range, i) => ({
        seat: i + 1,
        range,
        win: v[2 + 4 * i] ?? 0,
        tie: v[3 + 4 * i] ?? 0,
        equity: v[4 + 4 * i] ?? 0,
        error: v[5 + 4 * i] ?? 0,
      })),
    });
  }

  return (
    <div className={styles.workspace}>
      <form className={styles.panel} onSubmit={calculate} aria-labelledby="spot-heading">
        <h2 id="spot-heading">Spot</h2>
        {players.map((value, i) => (
          // Rows are only added or removed at the end, so the position is a stable key.
          // biome-ignore lint/suspicious/noArrayIndexKey: see above
          <div key={i} className={styles.field}>
            <label htmlFor={`player-${i}`}>Player {i + 1}</label>
            <input
              id={`player-${i}`}
              className={styles.input}
              value={value}
              onChange={(e) => setPlayers(players.map((p, j) => (j === i ? e.target.value : p)))}
              placeholder={i === 0 ? "AhAs" : "22+, ATs+, KQo"}
              spellCheck={false}
              autoComplete="off"
            />
          </div>
        ))}
        <div className={styles.actions}>
          <button
            type="button"
            className={styles.secondary}
            disabled={players.length >= 9}
            onClick={() => setPlayers([...players, ""])}
          >
            Add player
          </button>
          <button
            type="button"
            className={styles.secondary}
            disabled={players.length <= 2}
            onClick={() => setPlayers(players.slice(0, -1))}
          >
            Remove player {players.length}
          </button>
        </div>
        <div className={styles.field}>
          <label htmlFor="board">Board</label>
          <input
            id="board"
            className={styles.input}
            value={board}
            onChange={(e) => setBoard(e.target.value)}
            placeholder="Empty for preflop, or Kh 7c 2d"
            spellCheck={false}
            autoComplete="off"
          />
        </div>
        <button type="submit" className={styles.primary} disabled={busy}>
          {busy ? "Calculating…" : "Calculate"}
        </button>
        {error && (
          <p className={styles.error} role="alert">
            {error}
          </p>
        )}
      </form>

      {result && (
        <section className={styles.panel} aria-labelledby="result-heading">
          <h2 id="result-heading">Equity</h2>
          <p id="method" className={styles.method}>
            {result.exact
              ? `Exact: ${result.count.toLocaleString("en-US")} deals enumerated`
              : `Estimate: Monte Carlo over ${result.count.toLocaleString("en-US")} sampled deals (seed ${SEED}); ± is one standard error`}
          </p>
          <div className={styles.scroll}>
            <table className={styles.table} aria-labelledby="method">
              <thead>
                <tr>
                  <th scope="col">Player</th>
                  <th scope="col">Equity</th>
                  <th scope="col">Win</th>
                  <th scope="col">Tie</th>
                  <th scope="col">Hand or range</th>
                </tr>
              </thead>
              <tbody>
                {result.players.map((p) => (
                  <tr key={p.seat}>
                    <th scope="row">Player {p.seat}</th>
                    <td>
                      {formatShare(p.equity, 2)}
                      {!result.exact && (
                        <small className={styles.margin}>{formatMargin(p.error)}</small>
                      )}
                    </td>
                    <td>{formatShare(p.win, 2)}</td>
                    <td>{formatShare(p.tie, 2)}</td>
                    <td>{p.range}</td>
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

"use client";

import { createBrowserEnginePool, type EnginePool } from "@fieldtest/engine-wasm";
import { type FormEvent, useEffect, useRef, useState } from "react";
import { formatCombos, formatShare } from "../../../lib/format";
import styles from "./analyzer.module.css";

const RANKS = "AKQJT98765432";
const RANK_NAMES = "Ace King Queen Jack Ten Nine Eight Seven Six Five Four Three Two".split(" ");
const SUITS = [
  { code: "s", symbol: "♠", name: "spades" },
  { code: "h", symbol: "♥", name: "hearts" },
  { code: "d", symbol: "♦", name: "diamonds" },
  { code: "c", symbol: "♣", name: "clubs" },
] as const;
const STREETS = ["flop", "turn", "river"];
const LABELS: Record<string, string> = {
  StraightFlush: "Straight flush",
  Quads: "Quads",
  FullHouse: "Full house",
  Flush: "Flush",
  Straight: "Straight",
  Set: "Set",
  Trips: "Trips",
  TwoPair: "Two pair",
  Overpair: "Overpair",
  TopPair: "Top pair",
  Underpair: "Underpair",
  MiddlePair: "Middle pair",
  WeakPair: "Weak pair",
  AceHigh: "Ace high",
  Nothing: "No made hand",
  FlushDraw: "Flush draw",
  OpenEnded: "Open-ended straight draw",
  Gutshot: "Gutshot",
};
const ERRORS: Record<string, string> = {
  InvalidToken: "That range notation isn't recognised. Try 22+, ATs+, KQo or AhKh.",
  InvalidWeight: "Weights must be above 0 and at most 1, as in AKs:0.5.",
};

type Row = { name: string; weight: number; share: number };
type Analysis = { live: number; classes: Row[]; draws: Row[] };

/** Grid cell label: pairs on the diagonal, suited above it, offsuit below. */
function cellLabel(row: number, col: number): string {
  const [high, low] = [RANKS[Math.min(row, col)], RANKS[Math.max(row, col)]];
  return row === col ? `${high}${low}` : `${high}${low}${row < col ? "s" : "o"}`;
}

export function RangeAnalyzer() {
  const pool = useRef<EnginePool | null>(null);
  const [text, setText] = useState("");
  const [range, setRange] = useState("");
  const [grid, setGrid] = useState<number[]>(() => Array(169).fill(0));
  const [board, setBoard] = useState<string[]>([]);
  const [locked, setLocked] = useState(0);
  const [analysis, setAnalysis] = useState<Analysis | null>(null);
  const [keep, setKeep] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    pool.current = createBrowserEnginePool();
  }, []);

  useEffect(() => {
    pool.current?.call("gridWeights", range).then((r) => r.ok && setGrid([...r.value]));
  }, [range]);

  useEffect(() => {
    if (board.length < 3 || !pool.current) {
      setAnalysis(null);
      return;
    }
    let current = true;
    const engine = pool.current;
    Promise.all([
      engine.call("analysisLabels"),
      engine.call("analyzeRange", range, board.join(" ")),
    ]).then(([labels, values]) => {
      if (!current || !labels.ok || !values.ok) return;
      const rows = labels.value.map((name, i) => ({
        name,
        weight: values.value[1 + 2 * i] ?? 0,
        share: values.value[2 + 2 * i] ?? 0,
      }));
      setAnalysis({
        live: values.value[0] ?? 0,
        classes: rows.slice(0, 15),
        draws: rows.slice(15),
      });
    });
    return () => {
      current = false;
    };
  }, [range, board]);

  function showRange(result: { ok: true; value: string } | { ok: false; code: string }) {
    if (!result.ok) {
      setError(ERRORS[result.code] ?? `The engine refused this input (${result.code}).`);
      return;
    }
    setError(null);
    setRange(result.value);
    setText(result.value);
  }

  async function applyRange(event: FormEvent) {
    event.preventDefault();
    if (pool.current) showRange(await pool.current.call("normalizeRange", text));
  }

  async function toggleCell(label: string, weight: number) {
    if (pool.current)
      showRange(await pool.current.call("setClass", range, label, weight === 1 ? 0 : 1));
  }

  function toggleCard(card: string) {
    const at = board.indexOf(card);
    if (at >= locked) setBoard(board.filter((c) => c !== card));
    else if (at === -1 && board.length < 5) setBoard([...board, card]);
  }

  async function nextStreet() {
    if (!pool.current) return;
    showRange(await pool.current.call("filterRange", range, board.join(" "), keep.join(",")));
    setLocked(board.length);
    setKeep([]);
  }

  const street = STREETS[board.length - 3];
  const prompt =
    board.length < 3
      ? `Pick ${3 - board.length} more flop card${board.length === 2 ? "" : "s"}`
      : board.length === locked && board.length < 5
        ? `Pick the ${STREETS[board.length - 2]} card`
        : null;

  return (
    <div className={styles.workspace}>
      <section className={styles.panel} aria-labelledby="range-heading">
        <h2 id="range-heading">Range</h2>
        <form className={styles.rangeForm} onSubmit={applyRange}>
          <label htmlFor="range-text">Range</label>
          <input
            id="range-text"
            className={styles.rangeInput}
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="22+, ATs+, KQo, AKs:0.5"
            spellCheck={false}
            autoComplete="off"
          />
          <button type="submit" className={styles.primary}>
            Apply range
          </button>
        </form>
        {error && (
          <p className={styles.error} role="alert">
            {error}
          </p>
        )}
        <div className={styles.gridScroll}>
          <div className={styles.grid}>
            {grid.map((weight, i) => {
              const label = cellLabel(Math.floor(i / 13), i % 13);
              const state = weight === 1 ? "full" : weight > 0 ? "partial" : "none";
              return (
                <button
                  key={label}
                  type="button"
                  className={styles.cell}
                  data-weight={state}
                  aria-pressed={state === "full" ? true : state === "partial" ? "mixed" : false}
                  aria-label={
                    weight === 0
                      ? `${label}, not in range`
                      : `${label}, ${formatShare(weight)} in range`
                  }
                  onClick={() => toggleCell(label, weight)}
                >
                  <span>{label}</span>
                  {state === "partial" && <small>{formatShare(weight)}</small>}
                </button>
              );
            })}
          </div>
        </div>
      </section>

      <section className={styles.panel} aria-labelledby="board-heading">
        <h2 id="board-heading">Board</h2>
        <p className={styles.boardLine} aria-live="polite">
          {board.length === 0 ? (
            <span className={styles.muted}>No cards yet</span>
          ) : (
            board.map((card) => (
              <span key={card} className={styles.boardCard} data-suit={card[1]}>
                {card[0]}
                {SUITS.find((s) => s.code === card[1])?.symbol}
              </span>
            ))
          )}
        </p>
        {prompt && <p className={styles.prompt}>{prompt}</p>}
        <div className={styles.deck}>
          {SUITS.map((suit) => (
            <div key={suit.code} className={styles.suitRow}>
              {[...RANKS].map((rank, r) => {
                const card = `${rank}${suit.code}`;
                const at = board.indexOf(card);
                return (
                  <button
                    key={card}
                    type="button"
                    className={styles.card}
                    data-suit={suit.code}
                    aria-pressed={at !== -1}
                    aria-label={`${RANK_NAMES[r]} of ${suit.name}`}
                    disabled={(at !== -1 && at < locked) || (at === -1 && board.length >= 5)}
                    onClick={() => toggleCard(card)}
                  >
                    {rank}
                    {suit.symbol}
                  </button>
                );
              })}
            </div>
          ))}
        </div>
        {board.length > locked && (
          <button
            type="button"
            className={styles.secondary}
            onClick={() => setBoard(board.slice(0, locked))}
          >
            Clear {locked === 0 ? "board" : STREETS[locked - 2]}
          </button>
        )}
      </section>

      {analysis && (
        <section
          className={`${styles.panel} ${styles.results}`}
          aria-labelledby="breakdown-heading"
        >
          <h2 id="breakdown-heading">Breakdown on the {street}</h2>
          <table className={styles.table}>
            <caption>
              Exact: every live combo enumerated · {formatCombos(analysis.live)} live combos after
              card removal
            </caption>
            <thead>
              <tr>
                <th scope="col">Made hand</th>
                <th scope="col">Combos</th>
                <th scope="col">Share</th>
                {board.length < 5 && <th scope="col">Keep</th>}
              </tr>
            </thead>
            {[analysis.classes, analysis.draws].map((rows, group) => (
              <tbody key={group === 0 ? "classes" : "draws"}>
                {group === 1 && rows.some((r) => r.weight > 0) && (
                  <tr className={styles.groupRow}>
                    <th scope="colgroup" colSpan={4}>
                      Draws (a combo can hold several)
                    </th>
                  </tr>
                )}
                {rows
                  .filter((row) => row.weight > 0)
                  .map((row) => (
                    <tr key={row.name}>
                      <th scope="row">{LABELS[row.name] ?? row.name}</th>
                      <td>{formatCombos(row.weight)}</td>
                      <td>{formatShare(row.share)}</td>
                      {board.length < 5 && (
                        <td>
                          <input
                            type="checkbox"
                            className={styles.check}
                            aria-label={`Keep ${LABELS[row.name] ?? row.name}`}
                            checked={keep.includes(row.name)}
                            onChange={(e) =>
                              setKeep(
                                e.target.checked
                                  ? [...keep, row.name]
                                  : keep.filter((k) => k !== row.name),
                              )
                            }
                          />
                        </td>
                      )}
                    </tr>
                  ))}
              </tbody>
            ))}
          </table>
          {board.length < 5 && (
            <button
              type="button"
              className={styles.primary}
              disabled={keep.length === 0}
              onClick={nextStreet}
            >
              Keep selected and go to the {STREETS[board.length - 2]}
            </button>
          )}
        </section>
      )}
    </div>
  );
}

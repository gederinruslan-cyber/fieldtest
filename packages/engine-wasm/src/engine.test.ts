import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { beforeAll, expect, test } from "vitest";
import { initSync } from "../pkg/fieldtest_wasm.js";
import { type EngineCall, type EngineResult, handle } from "./handle.ts";

beforeAll(() => {
  initSync({ module: readFileSync(new URL("../pkg/fieldtest_wasm_bg.wasm", import.meta.url)) });
});

const RANGES = [
  "22+",
  "ATs+",
  "AKo",
  "AK",
  "KTs-K7s",
  "77-55",
  "AhKh",
  "QQ+,AKs:0.5",
  "22+,A2+,K2+,Q2+,J2+,T2+,92+,82+,72+,62+,52+,42+,32",
  "",
  "AA:0.3333333333333333,KQs:0.1,AhKd:0.7",
  "AA:0",
  "AA:1.5",
  "AKx",
  "AA-KQ",
  "AhAh",
];
const HANDS = [
  "As Ks Qs Js Ts",
  "9s Ks Qs Js Ts",
  "Ac 2d 3h 4s 5c",
  "2c 3d 4h 5s 6c",
  "As Ah Kc Kd Qs",
  "As Ks",
  "As As Kd Qc Jh",
  "As Ks Qs Js 1s",
];

/** Seeded distinct-card hands of 5 to 7 cards (a fixed LCG, so both sides see the same set). */
function seededHands(count: number): string[] {
  const deck = [..."23456789TJQKA"].flatMap((r) => [..."cdhs"].map((s) => r + s));
  let state = 20261028;
  const next = () => {
    state = (Math.imul(state, 1103515245) + 12345) >>> 0;
    return state;
  };
  return Array.from({ length: count }, (_, i) => {
    const cards = [...deck];
    return Array.from(
      { length: 5 + (i % 3) },
      () => cards.splice(next() % cards.length, 1)[0],
    ).join(" ");
  });
}

/** Seeded spots: five single combos and a 3- to 5-card board (overlaps test card removal). */
function seededSpots(count: number): [string, string][] {
  const hands = seededHands(count * 2);
  return Array.from({ length: count }, (_, i) => {
    const cards = (hands[2 * i] as string).split(" ");
    const range = [0, 2].map((j) => `${cards[j]}${cards[j + 1]}`).join(",");
    return [
      range,
      (hands[2 * i + 1] as string)
        .split(" ")
        .slice(0, 3 + (i % 3))
        .join(" "),
    ];
  });
}

const SPOTS: [string, string][] = [
  ["AA", "Ah 7c 2d"],
  ["AK", "Kh 7c 2d"],
  ["AhQh", "Kh 7h 2c"],
  ["98s", "Tc 7d 2s"],
  ["J9o", "Qc 8d 2s"],
  ["AhAd", "9c 8d 7s 6h"],
  ["22+,A2s+,KTo+:0.5", "Kh 9c 4d 4s 2h"],
  ["AA", "Ah 7c"],
  ["AKx", "Ah 7c 2d"],
  ...seededSpots(300),
];

/** Equity spots: exact postflop and multiway, seeded Monte Carlo preflop, refused input. */
const EQUITY: [string, string, number, number][] = [
  ["AhAs;KdKc", "Kh 7c 2d", 1000, 1],
  ["AhKh;QQ+,AKs:0.5", "Qh 7h 2c 3s", 1000, 1],
  ["22+,A2s+,K9s+,ATo+;22+,A2s+,K9s+,ATo+", "Kh 7c 2d", 1000, 1],
  ["AA,KK;QQ,JJ;TT,99", "2c 3d 4h 5s 9d", 1000, 1],
  ["AA;KK", "", 20000, 7],
  ["22+,A2+;KQ,AK,QQ+;AhKh", "", 20000, 8],
  ["22+;22+;22+;22+;22+;22+;22+;22+;22+", "", 5000, 9],
  ["AA", "", 1000, 1],
  ["AhAs;AhKd", "", 1000, 1],
  ["AA;KK", "Ah 7c", 1000, 1],
  ["AA;KK", "", 1, 1],
  ["AA;AKx", "", 1000, 1],
];

const CALLS: EngineCall[] = [
  { fn: "analysisLabels", args: [] },
  ...RANGES.flatMap((r): EngineCall[] => [
    { fn: "normalizeRange", args: [r] },
    { fn: "rangeSize", args: [r] },
    { fn: "gridWeights", args: [r] },
    { fn: "setClass", args: [r, "AKo", 0.5] },
  ]),
  ...[...HANDS, ...seededHands(3000)].flatMap((h): EngineCall[] => [
    { fn: "evaluateHand", args: [h] },
    { fn: "handCategory", args: [h] },
  ]),
  ...SPOTS.flatMap(([range, board]): EngineCall[] => [
    { fn: "analyzeRange", args: [range, board] },
    { fn: "filterRange", args: [range, board, "Set,TopPair,FlushDraw"] },
  ]),
  ...EQUITY.map((args): EngineCall => ({ fn: "equity", args })),
];

const NUMBERS = new Set(["rangeSize", "evaluateHand"]);
const LISTS = new Set(["analyzeRange", "gridWeights", "equity"]);

/** Reads a native `ok<TAB>value` / `err<TAB>code` line as the result WASM gives. */
function nativeResult(fn: string, line: string): EngineResult<unknown> {
  const [status, text = ""] = line.split("\t");
  if (status === "err") return { ok: false, code: text };
  if (NUMBERS.has(fn)) return { ok: true, value: Number(text) };
  if (LISTS.has(fn)) return { ok: true, value: text === "" ? [] : text.split(",").map(Number) };
  if (fn === "analysisLabels") return { ok: true, value: text.split(",") };
  return { ok: true, value: text };
}

test("WASM answers the reference set exactly as the native engine does", () => {
  const native = execFileSync("bash", ["../../scripts/engine.sh", "reference"], {
    input: CALLS.map((c) => [c.fn, ...c.args].join("\t")).join("\n"),
    encoding: "utf8",
  })
    .trimEnd()
    .split("\n");
  const wasm = CALLS.map((c) => {
    const r = handle(c);
    return r.ok && r.value instanceof Float64Array ? { ok: true, value: [...r.value] } : r;
  });
  // toEqual compares numbers with Object.is, so -0 and 0 differ.
  expect(wasm).toEqual(CALLS.map((c, i) => nativeResult(c.fn, native[i] ?? "")));
});

test("reference values come through WASM", () => {
  expect(handle({ fn: "rangeSize", args: ["QQ+,AKs:0.5"] })).toEqual({ ok: true, value: 20 });
  expect(handle({ fn: "rangeSize", args: ["22+"] })).toEqual({ ok: true, value: 78 });
  expect(handle({ fn: "normalizeRange", args: ["AKs,AQs,AJs,ATs"] })).toEqual({
    ok: true,
    value: "ATs+",
  });
  expect(handle({ fn: "handCategory", args: ["As Ks Qs Js Ts"] })).toEqual({
    ok: true,
    value: "StraightFlush",
  });
  const wheel = handle({ fn: "evaluateHand", args: ["Ac 2d 3h 4s 5c"] });
  const six = handle({ fn: "evaluateHand", args: ["2c 3d 4h 5s 6c"] });
  expect(wheel.ok && six.ok && wheel.value < six.value).toBe(true);
  const aces = handle({ fn: "equity", args: ["AhAs;KdKc", "", 1000, 1] });
  expect(aces.ok && [...(aces.value as Float64Array)].slice(0, 6)).toEqual([
    1, 1712304, 0.8106457731804633, 0.003818247227127893, 0.8125548967940273, 0,
  ]);
});

test("bad input comes back as an error code", () => {
  expect(handle({ fn: "normalizeRange", args: ["AKx"] })).toEqual({
    ok: false,
    code: "InvalidToken",
  });
  expect(handle({ fn: "evaluateHand", args: ["As Ks"] })).toEqual({
    ok: false,
    code: "InvalidHand",
  });
  expect(handle({ fn: "evaluateHand", args: ["As As Kd Qc Jh"] })).toEqual({
    ok: false,
    code: "InvalidHand",
  });
  expect(handle({ fn: "evaluateHand", args: ["As Ks Qs Js 1s"] })).toEqual({
    ok: false,
    code: "InvalidCard",
  });
});

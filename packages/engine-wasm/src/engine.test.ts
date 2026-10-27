import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { beforeAll, expect, test } from "vitest";
import { initSync } from "../pkg/fieldtest_wasm.js";
import { type EngineCall, handle } from "./handle.js";

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

const CALLS: EngineCall[] = [
  ...RANGES.flatMap((r) => [
    { fn: "normalizeRange", arg: r } as const,
    { fn: "rangeSize", arg: r } as const,
  ]),
  ...[...HANDS, ...seededHands(3000)].flatMap((h) => [
    { fn: "evaluateHand", arg: h } as const,
    { fn: "handCategory", arg: h } as const,
  ]),
];

test("WASM answers the reference set exactly as the native engine does", () => {
  const native = execFileSync("bash", ["../../scripts/engine.sh", "reference"], {
    input: CALLS.map((c) => `${c.fn}\t${c.arg}`).join("\n"),
    encoding: "utf8",
  })
    .trimEnd()
    .split("\n");
  const wasm = CALLS.map((c) => {
    const r = handle(c);
    if (!r.ok) return `err\t${r.code}`;
    return `ok\t${Object.is(r.value, -0) ? "-0" : r.value}`;
  });
  expect(wasm).toEqual(native);
});

test("reference values come through WASM", () => {
  expect(handle({ fn: "rangeSize", arg: "QQ+,AKs:0.5" })).toEqual({ ok: true, value: 20 });
  expect(handle({ fn: "rangeSize", arg: "22+" })).toEqual({ ok: true, value: 78 });
  expect(handle({ fn: "normalizeRange", arg: "AKs,AQs,AJs,ATs" })).toEqual({
    ok: true,
    value: "ATs+",
  });
  expect(handle({ fn: "handCategory", arg: "As Ks Qs Js Ts" })).toEqual({
    ok: true,
    value: "StraightFlush",
  });
  const wheel = handle({ fn: "evaluateHand", arg: "Ac 2d 3h 4s 5c" });
  const six = handle({ fn: "evaluateHand", arg: "2c 3d 4h 5s 6c" });
  expect(wheel.ok && six.ok && wheel.value < six.value).toBe(true);
});

test("bad input comes back as an error code", () => {
  expect(handle({ fn: "normalizeRange", arg: "AKx" })).toEqual({ ok: false, code: "InvalidToken" });
  expect(handle({ fn: "evaluateHand", arg: "As Ks" })).toEqual({ ok: false, code: "InvalidHand" });
  expect(handle({ fn: "evaluateHand", arg: "As As Kd Qc Jh" })).toEqual({
    ok: false,
    code: "InvalidHand",
  });
  expect(handle({ fn: "evaluateHand", arg: "As Ks Qs Js 1s" })).toEqual({
    ok: false,
    code: "InvalidCard",
  });
});

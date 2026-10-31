import * as engine from "../pkg/fieldtest_wasm.js";

/** The engine's browser functions; the WASM module must satisfy this shape. */
export interface EngineApi {
  normalizeRange(range: string): string;
  rangeSize(range: string): number;
  evaluateHand(cards: string): number;
  handCategory(cards: string): string;
  analysisLabels(): string[];
  analyzeRange(range: string, board: string): Float64Array;
  filterRange(range: string, board: string, keep: string): string;
  gridWeights(range: string): Float64Array;
  setClass(range: string, hand: string, weight: number): string;
  equity(players: string, board: string, samples: number, seed: number): Float64Array;
}

const api: EngineApi = engine;

export type EngineFunction = keyof EngineApi;
export type EngineValue<F extends EngineFunction> = ReturnType<EngineApi[F]>;
export type EngineCall = {
  [F in EngineFunction]: { fn: F; args: Parameters<EngineApi[F]> };
}[EngineFunction];
export type EngineResult<T> = { ok: true; value: T } | { ok: false; code: string };

/** Runs one call against the initialised WASM module; an engine error becomes its code. */
export function handle(call: EngineCall): EngineResult<EngineValue<EngineFunction>> {
  try {
    const fn = api[call.fn] as (...args: unknown[]) => EngineValue<EngineFunction>;
    return { ok: true, value: fn(...call.args) };
  } catch (code) {
    return { ok: false, code: String(code) };
  }
}

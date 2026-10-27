import * as engine from "../pkg/fieldtest_wasm.js";

/** What each engine function returns on success. */
export interface EngineResults {
  normalizeRange: string;
  rangeSize: number;
  evaluateHand: number;
  handCategory: string;
}

export type EngineFunction = keyof EngineResults;
export type EngineCall = { [F in EngineFunction]: { fn: F; arg: string } }[EngineFunction];
export type EngineResult<T> = { ok: true; value: T } | { ok: false; code: string };

/** Runs one call against the initialised WASM module; an engine error becomes its code. */
export function handle(call: EngineCall): EngineResult<EngineResults[EngineFunction]> {
  try {
    return { ok: true, value: engine[call.fn](call.arg) };
  } catch (code) {
    return { ok: false, code: String(code) };
  }
}

import type { EngineCall, EngineFunction, EngineResult, EngineResults } from "./handle.js";

export type EngineRequest = { id: number; call: EngineCall };
export type EngineResponse = { id: number; result: EngineResult<unknown> };

/** The part of a Web Worker the pool uses. */
export interface WorkerLike {
  postMessage(request: EngineRequest): void;
  addEventListener(type: "message", listener: (event: { data: EngineResponse }) => void): void;
}

export type EnginePool = {
  [F in EngineFunction]: (arg: string) => Promise<EngineResult<EngineResults[F]>>;
};

/** Spreads calls round-robin over the workers and hands each answer back to its caller. */
export function createEnginePool(workers: WorkerLike[]): EnginePool {
  const pending = new Map<number, (result: EngineResult<unknown>) => void>();
  let nextId = 0;
  for (const worker of workers) {
    worker.addEventListener("message", ({ data }) => {
      pending.get(data.id)?.(data.result);
      pending.delete(data.id);
    });
  }
  const call =
    <F extends EngineFunction>(fn: F) =>
    (arg: string) => {
      const id = nextId++;
      const worker = workers[id % workers.length] as WorkerLike;
      return new Promise<EngineResult<EngineResults[F]>>((resolve) => {
        pending.set(id, resolve as (result: EngineResult<unknown>) => void);
        worker.postMessage({ id, call: { fn, arg } as EngineCall });
      });
    };
  return {
    normalizeRange: call("normalizeRange"),
    rangeSize: call("rangeSize"),
    evaluateHand: call("evaluateHand"),
    handCategory: call("handCategory"),
  };
}

/** A pool of module Web Workers running the WASM engine, one per core up to four. */
export function createBrowserEnginePool(size = Math.min(navigator.hardwareConcurrency, 4)) {
  return createEnginePool(
    Array.from(
      { length: size },
      () => new Worker(new URL("./worker.ts", import.meta.url), { type: "module" }),
    ),
  );
}

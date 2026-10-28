import type { EngineApi, EngineCall, EngineFunction, EngineResult, EngineValue } from "./handle.ts";

export type EngineRequest = { id: number; call: EngineCall };
export type EngineResponse = { id: number; result: EngineResult<unknown> };

/** The part of a Web Worker the pool uses. */
export interface WorkerLike {
  postMessage(request: EngineRequest): void;
  addEventListener(type: "message", listener: (event: { data: EngineResponse }) => void): void;
}

export type EnginePool = {
  call<F extends EngineFunction>(
    fn: F,
    ...args: Parameters<EngineApi[F]>
  ): Promise<EngineResult<EngineValue<F>>>;
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
  return {
    call(fn, ...args) {
      const id = nextId++;
      const worker = workers[id % workers.length] as WorkerLike;
      return new Promise((resolve) => {
        pending.set(id, resolve as (result: EngineResult<unknown>) => void);
        worker.postMessage({ id, call: { fn, args } as EngineCall });
      });
    },
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

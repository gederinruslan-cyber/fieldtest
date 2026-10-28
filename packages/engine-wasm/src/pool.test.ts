import { expect, test } from "vitest";
import {
  createEnginePool,
  type EngineRequest,
  type EngineResponse,
  type WorkerLike,
} from "./pool.ts";

/** A worker that answers with its own name and the argument, after a per-call delay. */
function fakeWorker(name: string, delays: number[]): WorkerLike {
  let listener: (event: { data: EngineResponse }) => void = () => {};
  return {
    addEventListener: (_type, l) => {
      listener = l;
    },
    postMessage({ id, call }: EngineRequest) {
      const response: EngineResponse = {
        id,
        result: { ok: true, value: `${name}:${call.args[0]}` },
      };
      setTimeout(() => listener({ data: response }), delays.shift() ?? 0);
    },
  };
}

test("each caller gets its own answer when workers reply out of order", async () => {
  const pool = createEnginePool([fakeWorker("a", [30, 0, 10]), fakeWorker("b", [20, 0])]);
  const results = await Promise.all(
    ["1", "2", "3", "4", "5"].map((n) => pool.call("normalizeRange", n)),
  );
  expect(results.map((r) => r.ok && r.value)).toEqual(["a:1", "b:2", "a:3", "b:4", "a:5"]);
});

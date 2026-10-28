import init from "../pkg/fieldtest_wasm.js";
import { handle } from "./handle.ts";
import type { EngineRequest, EngineResponse } from "./pool.ts";

// Web Worker entry: the engine never runs on the main thread.
const ready = init();

self.onmessage = async ({ data }: MessageEvent<EngineRequest>) => {
  await ready;
  const response: EngineResponse = { id: data.id, result: handle(data.call) };
  self.postMessage(response);
};

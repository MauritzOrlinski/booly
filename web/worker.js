// Runs the WebAssembly solver off the main thread so the page stays responsive.
import init, { solve, restart_heuristics } from "./pkg/booly.js";

const ready = init().then(() => {
  self.postMessage({ type: "ready", heuristics: restart_heuristics() });
});

self.onmessage = async (event) => {
  const { id, dimacs, options } = event.data;
  await ready;
  const started = performance.now();
  try {
    const result = solve(dimacs, options);
    self.postMessage({ type: "result", id, result, millis: performance.now() - started });
  } catch (err) {
    self.postMessage({ type: "result", id, result: { status: "error", error: String(err) }, millis: performance.now() - started });
  }
};

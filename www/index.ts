import init, { greet } from "../wasm/pkg/wasm.js";

async function runWasm() {
  await init();
  greet();
}

runWasm();

console.log("Hello via Bun!");

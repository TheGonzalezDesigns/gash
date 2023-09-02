import init, { greet } from "../wasm/pkg/wasm.js";
import { animate } from "./modules/three";

async function runWasm() {
  await init();
  greet();
}

runWasm();

animate();

console.log("Hello via Bun!");

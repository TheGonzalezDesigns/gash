import init, { greet } from "./production/wasm.js";
import { animate } from "./modules/three";

async function runWasm() {
  await init();
  greet();
}

runWasm();

animate();

console.log("Hello via Bun!");

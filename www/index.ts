import init, { greet, find_path } from "./production/wasm.js";
import { animate, highlightRooms } from "./modules/three";

async function runWasm() {
  await init();
  greet();

  // Assuming the Rust function's name is 'find_path' and it returns an array of room indices
  const path = find_path();

  if (path) {
    highlightRooms(path);
  }
}

runWasm();
animate();

console.log("Hello via Bun!");

import init from "./production/wasm.js";
import * as WASM from "./production/wasm.js";
import * as draw from "./modules/three";

await init();

type DoorLock =
  | "LockedFromInside"
  | "LockedFromOutside"
  | "UnlockedFromInside"
  | "UnlockedFromOutside";

interface Door {
  lock: DoorLock;
}

interface Room {
  entry: Door;
  exit: Door;
}

interface RoomGrid {
  rooms: Room[];
}

async function WebAssembly() {
  WASM.greet();

  const roomGridSize = 100;
  const roomGrid = WASM.generate_random_rooms(roomGridSize);

  const startRoomIndex = 0;
  const endRoomIndex = 99; // Example for last room in a 10x10 grid
  const path = WASM.find_path(startRoomIndex, roomGrid);
  //const path = WASM.find_path(startRoomIndex, endRoomIndex, roomGrid);

  if (path) {
    draw.highlightRooms(path);
  }
}

draw.animate();
await WebAssembly();
console.log("Hello via Bun!");

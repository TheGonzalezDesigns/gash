import init, { greet, wasm_find_path } from "./production/wasm.js";
import { animate, highlightRooms } from "./modules/three";

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

const sampleRoomGrid: RoomGrid = {
  rooms: [
    {
      entry: { lock: "UnlockedFromInside" },
      exit: { lock: "UnlockedFromOutside" },
    },
    {
      entry: { lock: "LockedFromInside" },
      exit: { lock: "LockedFromOutside" },
    },
    // ... more rooms as needed
  ],
};

async function runWasm() {
  await init();
  greet();

  const startRoomIndex = 0;
  const path = wasm_find_path(startRoomIndex, sampleRoomGrid);

  if (path) {
    highlightRooms(path);
  }
}

runWasm();
animate();
console.log("Hello via Bun!");

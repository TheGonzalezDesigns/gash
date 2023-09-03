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
  const roomGrid: RoomGrid = WASM.generate_random_rooms(roomGridSize);

  const startRoomIndex = 0;
  const endRoomIndex = 99; // Example for last room in a 10x10 grid
  const path = WASM.find_path(startRoomIndex, endRoomIndex, roomGrid);

  if (path) {
    visualizePath(path);
  }
}

function visualizePath(path: number[]) {
  for (let i = 0; i < path.length; i++) {
    setTimeout(() => {
      // Setting the color to yellow for traversal
      draw.updateBlockColor(path[i], "yellow");
      if (i === path.length - 1) {
        // If it's the last room in the path, set its color to green
        draw.updateBlockColor(path[i], "green");
      }
    }, i * 500); // 500ms delay between each block update for visualization
  }
}

draw.animate();
await WebAssembly();
console.log("Hello via Bun!");

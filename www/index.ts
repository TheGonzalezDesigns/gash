import init, {
  greet,
  wasm_find_path,
  generate_random_rooms,
} from "./production/wasm.js";
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

const roomGridSize = 100; // Example for a 10x10 grid
const roomGrid = generate_random_rooms(roomGridSize);

async function runWasm() {
  await init();
  greet();

  const roomGridSize = 100;
  const roomGrid = generate_random_rooms(roomGridSize);

  const startRoomIndex = 0;
  const endRoomIndex = 99; // Example for last room in a 10x10 grid
  const path = wasm_find_path(startRoomIndex, endRoomIndex, roomGrid);

  if (path) {
    highlightRooms(path);
  }
}

runWasm();
animate();
console.log("Hello via Bun!");

import init from "./production/wasm.js";
import * as WASM from "./production/wasm.js";

await init();

describe("WASM.greet function", () => {
  test("it should WASM.greet", () => {
    expect(WASM.greet()).toBe("Hello, wasm!");
  });
});

describe("WASM.find_path function", () => {
  const sampleValidRoomGrid = {
    rooms: [
      {
        entry: { lock: "UnlockedFromInside" },
        exit: { lock: "UnlockedFromOutside" },
      },
      {
        entry: { lock: "UnlockedFromInside" },
        exit: { lock: "UnlockedFromOutside" },
      },
    ],
  };

  test("it should find a path given valid room grid", () => {
    const result = WASM.find_path(0, 1, sampleValidRoomGrid); // added end room
    expect(result.length).toBeGreaterThan(0);
  });

  test("it should return null when no path exists", () => {
    const sampleInvalidRoomGrid = {
      rooms: [
        {
          entry: { lock: "LockedFromInside" },
          exit: { lock: "LockedFromOutside" },
        },
        {
          entry: { lock: "LockedFromInside" },
          exit: { lock: "LockedFromOutside" },
        },
      ],
    };
    const result = WASM.find_path(0, 1, sampleInvalidRoomGrid); // added end room
    expect(result).toBeNull();
  });

  test("it should handle out of bounds start room index", () => {
    const result = WASM.find_path(10, 1, sampleValidRoomGrid); // 10 is out of bounds for the sample grid
    expect(result).toBeNull();
  });
});

describe("WASM.generate_random_rooms function", () => {
  const num_rooms = 5;
  const result = WASM.generate_random_rooms(num_rooms);

  test("it should generate the specified number of rooms", () => {
    expect(result.rooms.length).toBe(num_rooms);
  });

  test("each room should have entry and exit doors", () => {
    result.rooms.forEach((room) => {
      expect(room).toHaveProperty("entry");
      expect(room).toHaveProperty("exit");
    });
  });

  test("each door should have a valid lock state", () => {
    const validLockStates = [
      "LockedFromInside",
      "LockedFromOutside",
      "UnlockedFromInside",
      "UnlockedFromOutside",
    ];
    result.rooms.forEach((room) => {
      expect(validLockStates).toContain(room.entry.lock);
      expect(validLockStates).toContain(room.exit.lock);
    });
  });
});

// Add more tests as needed for other functions

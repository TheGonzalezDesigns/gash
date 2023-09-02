import { greet, wasm_find_path, generate_random_rooms } from "./index";

describe("greet function", () => {
  test("it should greet", () => {
    expect(greet()).toBe("Hello, wasm!");
  });
});

describe("wasm_find_path function", () => {
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
    const result = wasm_find_path(0, sampleValidRoomGrid);
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
    const result = wasm_find_path(0, sampleInvalidRoomGrid);
    expect(result).toBeNull();
  });

  test("it should handle out of bounds start room index", () => {
    const result = wasm_find_path(10, sampleValidRoomGrid); // 10 is out of bounds for the sample grid
    expect(result).toBeNull();
  });
});

describe("generate_random_rooms function", () => {
  const num_rooms = 5;
  const result = generate_random_rooms(num_rooms);

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

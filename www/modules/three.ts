import * as THREE from "three";

// Create the scene
const scene = new THREE.Scene();

// Set up the camera
const camera = new THREE.PerspectiveCamera(
  75,
  window.innerWidth / window.innerHeight,
  0.1,
  1000
);
camera.position.z = 15;
camera.position.y = 5;
camera.lookAt(new THREE.Vector3(5, 5, 0)); // Assuming a 10x10 grid centered around (5,5)

// Set up the renderer
const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Add lighting
const ambientLight = new THREE.AmbientLight(0x404040, 1);
scene.add(ambientLight);
const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
directionalLight.position.set(1, 1, 1);
scene.add(directionalLight);

// Define room geometry and material
const geometry = new THREE.BoxGeometry(0.9, 0.9, 0.9); // Slightly less than 1 to see gaps between rooms
const material = new THREE.MeshBasicMaterial({ color: 0x808080 }); // Set initial color to grey

// Create a 2D grid of rooms and add them to the scene
const rooms: THREE.Mesh[] = [];
for (let i = 0; i < 10; i++) {
  // For a 10x10 grid
  for (let j = 0; j < 10; j++) {
    const roomMesh = new THREE.Mesh(geometry, material.clone());
    roomMesh.position.set(i, j, 0);
    rooms.push(roomMesh);
    scene.add(roomMesh);
  }
}

// This function takes an array of room indices and highlights them
export function highlightRooms(path: number[]) {
  let index = 0;
  const highlightInterval = setInterval(() => {
    if (index < path.length) {
      rooms[path[index]].material.color.set(0xff0000); // Highlight with red color
      index++;
    } else {
      clearInterval(highlightInterval);
    }
  }, 200); // 200ms delay for example
}

// Render the scene continuously
export function animate() {
  requestAnimationFrame(animate);
  renderer.render(scene, camera);
}

// Array to store the blocks for future reference
const blocks = rooms;

export function updateBlockColor(index: number, color: string) {
  if (blocks[index]) {
    blocks[index].material.color.set(color);
  }
}

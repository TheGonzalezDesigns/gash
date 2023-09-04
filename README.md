# ⚡️ GASH: Real-time Pathfinding Visualization

![Banner Image](https://i.imgur.com/uCUmyGs.png)

> Visualize pathfinding algorithms in real time!

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/TheGonzalezDesigns/gash) [![WASM](https://img.shields.io/badge/WASM-powered-blue)](https://webassembly.org/) [![Three.js](https://img.shields.io/badge/Three.js-visuals-orange)](https://threejs.org/) 

**Note**: GASH is currently in early beta. We are testing with mock data instead of optimized algorithms on real data. Optimizations and updates coming soon!

---

## 🌟 Features

- **3D Visualization**: Experience the arbitrage pathfinding algorithm unfold in a 3D grid.
- **WebAssembly**: Harnessing the power of Rust compiled to WebAssembly for swift pathfinding.
- **Dynamic Grids**: Spawn random room grids with distinct locks to challenge the pathfinding algorithm.

---

## 🛠️ Installation & Set Up

1. **Clone the repository**

```bash
git clone https://github.com/TheGonzalezDesigns/gash.git
```

2. **Navigate to the directory**

```bash
cd gash
```

3. **Run the project using Docker** (Ensure you have Docker and Docker Compose installed)

```bash
docker-compose up
```

---

## 🕹️ Usage

After initiating the project, head to `http://localhost:9876` in your browser. Watch your pathfinder in action.

---

## 🔍 How It Works

1. **Rooms & Doors**: Each cell signifies a room equipped with an entry and an exit door, both bearing unique lock statuses.
2. **Pathfinding Algorithm**: An in-house algorithm is employed to delineate a path from the start to the destination room.
3. **Visualization**: Observe the algorithm in action as it spotlights the path in a real-time simulation.

---

## 🤝 Contributing

Your input is invaluable! We welcome contributions, queries, and feature proposals. Check out [CONTRIBUTING.md](./CONTRIBUTING.md) for more.

---

## 📜 License

This project follows the [MIT](./LICENSE) licensing protocol.

---

## 💌 Contact

- **GitHub**: [@TheGonzalezDesigns](https://github.com/TheGonzalezDesigns)
- **Email**: TheGonzalezDesigns@gmail.com

---


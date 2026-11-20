# Rust Defender

A 2D arcade shooter game built with **Rust** and **Macroquad**.

Rust Defender is a small game development project created to practice Rust programming, game loops, collision detection, object management, and modular software design.

## 🎮 Game Features

- **Player Movement** – Move the player using keyboard controls.
- **Shooting System** – Fire projectiles to destroy incoming enemies.
- **Enemy Spawning** – Enemies spawn during gameplay.
- **Collision Detection** – Detect collisions between bullets, enemies, and the player.
- **Scoring System** – Earn points by destroying enemies.
- **Main Menu** – Display game instructions and a clickable Start Game button.
- **Game Over Screen** – Display the final score when the player is defeated.
- **Restart System** – Restart the game using the R key or the Play Again button.
- **Exit Option** – Exit the game through the Exit Game button.

## 🕹️ Controls

| Key / Action | Function |
|---|---|
| Arrow Keys | Move the player |
| Space | Shoot |
| R | Restart after Game Over |
| Left Mouse Click | Interact with menu buttons |

## 🛠️ Technologies

- **Rust** – Main programming language
- **Macroquad** – 2D game framework
- **Cargo** – Dependency management and build system

## 📁 Project Structure

```text
Rust-Defender/
│
├── src/
│   ├── main.rs
│   ├── game.rs
│   ├── ui.rs
│   ├── player.rs
│   ├── enemy.rs
│   ├── bullet.rs
│   └── collision.rs
│
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

### Module Overview

| Module | Responsibility |
|---|---|
| `main.rs` | Application entry point and game loop |
| `game.rs` | Game state, gameplay logic, and object management |
| `ui.rs` | Main menu, buttons, and Game Over interface |
| `player.rs` | Player movement and behavior |
| `enemy.rs` | Enemy behavior and movement |
| `bullet.rs` | Projectile behavior |
| `collision.rs` | Collision detection utilities |

## 🚀 Getting Started

### Prerequisites

- Rust
- Cargo

Install Rust from: https://www.rust-lang.org/tools/install

### Run the Game

Clone the repository:

```bash
git clone https://github.com/Reid-0100/Rust-Defender.git
```

Navigate to the project directory:

```bash
cd Rust-Defender
```

Run the game:

```bash
cargo run
```

## 📚 Learning Objectives

This project provides hands-on practice with:

- Rust structs, enums, and methods
- Ownership and mutable references
- Modular code organization
- Game loops and frame updates
- Collision detection
- Random number generation
- UI interaction and input handling
- Managing game states

## 🔧 Future Improvements

- Improve enemy spawning and difficulty progression.
- Add more enemy types.
- Improve game animations and visual effects.
- Add sound effects and background music.
- Further improve UI and gameplay experience.

## 👨‍💻 Author

**Junyi Ma**

GitHub: https://github.com/Reid-0100

---

*This project is an ongoing personal learning project. More features and improvements may be added in future updates.*
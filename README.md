# Bevy 3D Resource Gathering Game

A modular 3D resource gathering game built with [Bevy](https://bevyengine.org/) and [bevy_rapier3d](https://github.com/dimforge/bevy_rapier).

## Features
- **3D third-person camera**: Smoothly follows and orbits the player, supports mouse-based rotation and zoom.
- **Camera-relative movement**: WASD moves the player relative to the camera's facing direction.
- **Resource gathering**: Walk up to trees or rocks, face them, and press `E` to gather resources.
- **Inventory system**: Tracks gathered resources and displays them in a HUD.
- **Modular codebase**: All major systems are separated into modules for clarity and extensibility.

## Controls
- **WASD**: Move the player (relative to camera)
- **Mouse drag (left or right button)**: Rotate camera around player
- **Mouse wheel**: Zoom camera in/out and adjust height
- **E**: Gather resource (when close and facing a tree or rock)
- **ESC**: Close the game window

## Code Structure
- `src/main.rs`: Bevy app setup, plugin and system registration
- `src/components/types.rs`: Shared components and resource types
- `src/systems/input.rs`: Handles keyboard input, stores movement direction
- `src/systems/movement.rs`: Moves the player, makes movement camera-relative, rotates player
- `src/systems/camera.rs`: Third-person camera follow and orbit system
- `src/systems/resources.rs`: Gathering logic and inventory tracking
- `src/systems/entities.rs`: Spawning logic for player, trees, rocks, ground, camera, and light
- `src/systems/ui.rs`: HUD display for inventory

## Getting Started
1. Install [Rust](https://rustup.rs/) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Clone this repository
3. Run `cargo run` in the project directory

## Extending the Game
- Add new resource types by extending the `ResourceType` enum and updating the spawn logic.
- Add new player abilities, skills, or UI elements by creating new systems and components.
- The modular structure makes it easy to add new features or refactor existing ones.

## License
MIT 
# Rust FOG OF WAR Game
This is a simple game implemented in Rust using the Piston game engine. The game creates a 2D grid-based world where the player can move around and explore the map. The visibility range of the player is limited, and the fog of war is used to hide unexplored areas.

## Features
- Grid-based world: The game world is represented as a grid of tiles, where each tile can be explored or unexplored.
- Player movement: The player can move around the map using the keyboard keys (W, A, S, D).
- Fog of war: Unexplored areas are hidden behind a fog of war, which gradually reveals as the player moves closer to them.
- Visibility range: The player has a limited visibility range, and tiles outside this range are not visible.
- Dynamic fog opacity: The opacity of the fog of war is dynamically adjusted based on the player's distance from each tile.
- Interaction: To interact with an objet (in our case its the blue square found by exploring the map) press (SPACE]) on your keyboard.


## Installation

To run the game, you need to have Rust installed on your system. Follow these steps to get started:

- Clone the repository:
```bash
  $ git clone https://github.com/Valery-a/fog-of-war.git
```
- Navigate to the project directory:
```bash
  $ cd fog-of-war
```
- Build and run the game using Cargo:
```bash
  $ cargo run
```

# Customization
Feel free to customize the game according to your preferences. Here are a few suggestions:

- Modify the SCREEN_WIDTH and SCREEN_HEIGHT constants to change the size of the game window.
- Adjust the TILE_SIZE constant to change the size of each tile in the grid.
- Experiment with different values for MAX_VISIBILITY_RANGE and MIN_FOG_OPACITY to alter the visibility and fog effects.
- Change the colors used for tiles, fog, and the player by modifying the corresponding constants (SHADOW_COLOR, EXPLORED_COLOR, and [1.0, 0.0, 0.0, 1.0] respectively).
- Add additional features, such as obstacles, enemies, or power-ups, to make the game more engaging.
- Give a start to the creator of this repository
# Acknowledgements
This game was created with the help of the following Rust crates:

- piston: A modular game engine for Rust.
- piston_window: A library for creating window applications in Piston.
## License

[MIT](https://choosealicense.com/licenses/mit/)

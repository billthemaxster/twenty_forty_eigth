# Game Structure


## Structs
- Tile
    - Value : int
    - Colour : Colour enum
- Game
    - Tile[][]
    - Score
- Engine
    - Game instance

## Enums
- MoveDirection 
    - left
    - right
    - up
    - down
- 


## Features

### Colour output
Leave colours until after v1 there's a crate called [colored](https://crates.io/crates/colored).

### Command line inputs
Possibly it might be worth having commandline inputs on startup for things, possibly look at using [clap-rs](https://github.com/clap-rs/clap/tree/v2.33.0).
# Slimefall

A short platformer game made using the bevy engine for LD48 (72 hours).

Build scripts (post jam) from https://github.com/NiklasEi/bevy_game_template

## Running

Desktop (native)

```bash
cargo run --features native
```

Web

```bash
# cargo install cargo-make
cargo make serve
```

## Done (Jam Version)

- [x] update message when player can exit
- [x] can't walk through vertical walls
- [x] game over state if y == 0 and slime_target > 0
- [x] game over if light = 0
- [x] progress to next level when finishing old level
- [x] display level in UI
- [x] BUG: levels don't colour when progressing
- [x] proper game over screen
- [x] fix colours to be more thematic
- [x] improve movement
- [x] liquid slime
- [x] flavour text / tutorials
- [x] 2 levels
- [x] victory condition + text
- [x] can't jump through floors - may require physics
- [x] BUG sinks and spawners weren't despawning between levels
- [x] 3 levels
- [x] sound effects

## Post jam 

- [x] make lighting a shader
- [x] HTML5 / WASM / WebGL build
- [ ] add some more interest in the background
- [ ] levels loaded from image
- [ ] procgen levels
- [ ] jump animations
- [ ] run animations
- [ ] physics based character controller
- [ ] solid slime should fall through liquid
- [ ] destructable terrain
- [ ] throwable flares
- [ ] r to reset at any time
- [ ] X lives before game over (i.e. maybe take some slime to respawn?)

## LICENSE

MIT / Apache dual license for source code.

Assets are public domain.

## CHANGELOG

### 1.3.0 (post jam)

- A slightly better character controller

### 1.2.0 (post jam)

- Get HTML5 (wasm/WebGL) and native builds both working

### 1.1.2 (post jam)

- Add conditional WASM dependencies for WebGL build

### 1.1.1 (post jam)

- Fixes to MacOS build script I hope
- Add conditional WASM dependencies for WebGL build

### 1.1.0 (post jam)

- Shaders for lighting
- Attempt at cross-platform build scripts courtesy of https://github.com/NiklasEi/bevy_game_template

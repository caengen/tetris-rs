# tetris-rs

Tetris in Rust & Macroquad

[![wasm build](https://github.com/caengen/tetris-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/caengen/tetris-rs/actions/workflows/rust.yml)

Run `cargo run`.

_(Press `G` in-game for debug mode.)_

![Demo](https://github.com/caengen/tetris-rs/blob/master/demo/demo.gif)

### Keybindings

| Key   | Action           |
| ----- | ---------------- |
| Left  | Move left        |
| Right | Move right       |
| Up    | Rotate clockwise |
| Down  | Soft drop        |
| Space | Hard drop        |
| C     | Hold tetromino   |
| P     | Pause game       |
| R     | Reset game       |
| G     | Debugger         |

### Implementation

- [x] T-Spin
- [x] Next piece view
- [x] Hold piece view
- [x] Score tracker

- [x] Holding
- [x] Wall kicks
- [x] Delayed Auto-Shift
- [x] Super Rotation System (SRS)
- [x] Initial Hold System (IHS)
- [ ] Initial Rotation System (IRS) 🤔?
- [ ] Counter-clockwise rotation (Z key?)
- [x] Frame count based timers
- [x] Soft drop
- [x] Hard drop
- [x] Soft locking
- [-] Floor kick. Need to fix infinity kicking
- [x] Ghost piece

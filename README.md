# Trail of Hunger

Top-down 2D game where you control a group of nomads to hunt and exhaust the ecosystem in each level.

## Goal

- You win the level when there are no animals left.
- You lose if hunger reaches 0.

## Controls

- Enter: advance / select
- Esc: pause (in-game)
- Left click: select nomads (click or drag)
- Right click: order move / hunt / eat (depending on the target)

## Run

```bash
cargo run
```

## Build

### Linux

```bash
cargo build --release
```

The executable will be at:

```
target/release/trail_of_hunger
```

### Windows (cross-compile from Linux)

Install the Windows target and linker:

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
```

Then build:

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

The executable will be at:

```
target/x86_64-pc-windows-gnu/release/trail_of_hunger.exe
```

Make sure to distribute the `assets/` folder alongside the executable:

```
trail_of_hunger.exe
assets/
└── fonts/
    └── alagard.ttf
```

## Notes

- Made for a <a href="https://itch.io/jam/mini-jam-207-primal">Mini Jam 207: Primal</a>.
- Limitation: Become the villain

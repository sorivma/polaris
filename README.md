# Polaris

**Cold War grand strategy game** — inspired by Hearts of Iron 4, set in 1945–1991.

## Stack

| Layer | Crate |
|---|---|
| Rendering & Windowing | `macroquad` |
| ECS | `hecs` |
| Scripting / Modding | `mlua` (LuaJIT) |
| UI (immediate mode) | `egui` + `egui-macroquad` |
| Serialization | `serde` + `serde_json` + `rkyv` |
| Math | `glam` |
| Networking | `laminar` |

## Development

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Lint
cargo clippy

# Release build
cargo build --release
```

## Project Structure

```
src/
├── main.rs              # Entry point, game loop
├── simulation/          # Pure simulation — tick, ECS, AI, combat, diplomacy
├── rendering/           # Province map, overlays, globe
├── ui/                  # egui panels, debug UI
├── audio/               # Music, SFX
└── network/             # Lockstep multiplayer
```

## License

MIT

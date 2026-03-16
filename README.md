# Snakes & Ladders (Rust)

Fresh Rust CLI boilerplate for Snakes and Ladders.

## Run

```bash
cargo run
```

At startup, enter your player name (the name `snake` is blocked).
Press ENTER to roll. Type `q` to quit.

Mechanics:
- Ladders give: land on a ladder to climb up.
- Snakes take: land on a snake to slide down.
- A six-sided dice determines each move.
- Reach position 100 to win.

currently out of juice

## Test

```bash
cargo test
```

## Optional format check

```bash
cargo fmt --check
```

## Notes

- `game.sh` is kept as the original bash proof-of-concept.
- Rust boilerplate lives in `src/` and is structured for easy extension.

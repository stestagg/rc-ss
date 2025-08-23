# Repository Guidance

This project implements a custom RC car controller and receiver using ESP32-H2 chips.
It contains three crates:

- `core` – shared types, utilities and protocol definitions.
- `transmitter` – binary that runs on the hand-held controller.
- `receiver` – binary that runs on the vehicle.

## Architecture & Goals
- Use raw 802.15.4 via [`esp-ieee802154`](https://github.com/esp-rs/esp-ieee802154) for all radio communication.
- Define a simple, reliable control protocol in the `core` crate and reuse it everywhere to avoid duplication.
- The transmitter reads two analogue sticks and any buttons, packages state into control packets and sends them at a fixed rate.
- The receiver decodes control packets and drives hardware:
  - PWM for an ESC (throttle) and a servo (steering).
  - Additional outputs such as headlights may be added later.
- Implement a failsafe: if the receiver does not get packets for a short period it must enter a neutral/stop state.

## Coding Conventions
- Rust edition 2021, `no_std` throughout.
- Keep the code simple and explicit; minimise clever abstractions.
- Share logic through the `core` crate rather than duplicating code.
- Use `defmt` for logging; avoid `println!`/`debug!` etc.
- Prefer small functions and modules over large files.

## Testing & CI
- Every change must compile for `riscv32imac-unknown-none-elf`.
  Run `cargo check --target riscv32imac-unknown-none-elf` from the workspace root.
- Add further tests or examples when practical.

## Backlog
High level work items live in the top-level `backlog/` directory as numbered `*.md` files. Each file represents a task for moving the project towards a fully functional RC system.

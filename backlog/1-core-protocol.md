# Core protocol and shared library

- Flesh out `ControlPacket` with additional fields such as buttons, trim values and checksum if needed.
- Implement serialization/deserialization helpers and packet constants in `core`.
- Define packet rates, value ranges and failsafe defaults.
- Document the packet layout so transmitter and receiver stay in sync.

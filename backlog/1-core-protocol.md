# Core protocol and shared library

- Flesh out `ControlPacket` with a u bitflag field (for say buttons in the future, just have headlight flag for now) and checksum value
- Implement serialization/deserialization helpers and packet constants in `core`.
- Define packet rates, value ranges and failsafe defaults.
- Document the packet layout so transmitter and receiver stay in sync.

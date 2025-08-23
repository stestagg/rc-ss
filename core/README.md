# Core protocol

The `ControlPacket` used for communication between transmitter and receiver has the
following layout (all fields little endian):

| bytes | field     | description                                     |
|-------|-----------|-------------------------------------------------|
| 0-1   | throttle  | `i16` throttle position                         |
| 2-3   | steering  | `i16` steering position                         |
| 4     | flags     | `u8` bitfield (bit0 = headlight)                |
| 5     | checksum  | `u8` sum of bytes 0-4, wrapping on overflow     |

Constants provided by the core crate:

- `ControlPacket::RATE_HZ` — packets are sent at this rate.
- `ControlPacket::THROTTLE_MIN`/`MAX` and `STEERING_MIN`/`MAX` — valid ranges.
- `ControlFlags::HEADLIGHT` — bit flag for headlights.
- `ControlPacket::FAILSAFE` — neutral packet used when communication is lost.

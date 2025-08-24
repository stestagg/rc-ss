# Transmitter input and packet sender

- Read analogue stick positions for throttle and steering. (pick the best pins)
- Map stick readings into `ControlPacket` values with trim offsets.
- Periodically send the current `ControlPacket` over the radio.
- Handle buttons for auxiliary features or failsafe reset.

# Receiver motor and servo control

- Configure PWM outputs for ESC (throttle) and servo (steering).
- Receive `ControlPacket` frames and apply values to PWM channels.
- Provide an optional output for headlights or other accessories.
- Initialise all outputs to a safe neutral state at startup.

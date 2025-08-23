# Failsafe logic

- Track time since the last valid packet was received.
- If no packet arrives within the timeout, set throttle and steering to neutral and disable outputs.
- Provide logging/LED indication when failsafe is active.
- Automatically resume normal operation when packets return.

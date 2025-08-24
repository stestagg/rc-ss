# 802.15.4 radio transport

- Integrate [`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio) and bring up raw 802.15.4 communication.
- Provide a small radio module (likely in `core`) with send/receive APIs.
- Use async functions to transmit and receive control packets.
- Prototype packet exchange between two boards.

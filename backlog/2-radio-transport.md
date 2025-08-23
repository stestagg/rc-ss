# 802.15.4 radio transport

- Integrate `esp-ieee802154` and bring up raw 802.15.4 communication.
- Provide a small radio module (likely in `core`) with send/receive APIs.
- Use async functions to transmit and receive control packets.
- Prototype packet exchange between two boards.

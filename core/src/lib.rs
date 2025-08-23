#![no_std]

use defmt::Format;

/// Control data shared between transmitter and receiver
#[derive(Clone, Copy, Format)]
pub struct ControlPacket {
    pub throttle: i16,
    pub steering: i16,
}

impl ControlPacket {
    pub const LEN: usize = 4;

    pub fn to_bytes(self) -> [u8; Self::LEN] {
        let t = self.throttle.to_le_bytes();
        let s = self.steering.to_le_bytes();
        [t[0], t[1], s[0], s[1]]
    }

    pub fn from_bytes(bytes: [u8; Self::LEN]) -> Self {
        let throttle = i16::from_le_bytes([bytes[0], bytes[1]]);
        let steering = i16::from_le_bytes([bytes[2], bytes[3]]);
        Self { throttle, steering }
    }
}

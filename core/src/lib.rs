#![no_std]

use bitflags::bitflags;
use defmt::Format;

/// Control data shared between transmitter and receiver.
///
/// Packet layout (little endian):
///
/// ```text
/// 0-1  throttle  i16
/// 2-3  steering  i16
/// 4    flags     u8  (bit 0 = headlight)
/// 5    checksum  u8  (sum of bytes 0-4, wrapping)
/// ```
bitflags! {
    #[derive(Format)]
    pub struct ControlFlags: u8 {
        /// Headlight toggle.
        const HEADLIGHT = 0x01;
    }
}
#[derive(Clone, Copy, Format)]
pub struct ControlPacket {
    /// Throttle position.
    pub throttle: i16,
    /// Steering position.
    pub steering: i16,
    /// Miscellaneous buttons or toggles.
    pub flags: ControlFlags,
    /// Checksum of all preceding bytes.
    pub checksum: u8,
}

impl ControlPacket {
    /// Total size of a serialized packet in bytes.
    pub const LEN: usize = 6;

    /// Packets are sent at this rate.
    pub const RATE_HZ: u32 = 50;
    /// Minimum throttle value.
    pub const THROTTLE_MIN: i16 = -1000;
    /// Maximum throttle value.
    pub const THROTTLE_MAX: i16 = 1000;
    /// Minimum steering value.
    pub const STEERING_MIN: i16 = -1000;
    /// Maximum steering value.
    pub const STEERING_MAX: i16 = 1000;
    /// Neutral/failsafe packet.
    pub const FAILSAFE: Self = Self {
        throttle: 0,
        steering: 0,
        flags: ControlFlags::empty(),
        checksum: 0,
    };

    /// Create a new packet and compute its checksum.
    pub fn new(throttle: i16, steering: i16, flags: ControlFlags) -> Self {
        let mut pkt = Self {
            throttle,
            steering,
            flags,
            checksum: 0,
        };
        pkt.checksum = pkt.calc_checksum();
        pkt
    }

    fn calc_checksum(&self) -> u8 {
        let t = self.throttle.to_le_bytes();
        let s = self.steering.to_le_bytes();
        t.iter()
            .chain(s.iter())
            .fold(self.flags.bits(), |acc, b| acc.wrapping_add(*b))
    }

    /// Serialize the packet to raw bytes.
    pub fn to_bytes(self) -> [u8; Self::LEN] {
        let t = self.throttle.to_le_bytes();
        let s = self.steering.to_le_bytes();
        let checksum = self.calc_checksum();
        [t[0], t[1], s[0], s[1], self.flags.bits(), checksum]
    }

    /// Deserialize a packet from raw bytes, validating the checksum.
    pub fn from_bytes(bytes: [u8; Self::LEN]) -> Option<Self> {
        let pkt = Self {
            throttle: i16::from_le_bytes([bytes[0], bytes[1]]),
            steering: i16::from_le_bytes([bytes[2], bytes[3]]),
            flags: ControlFlags::from_bits_retain(bytes[4]),
            checksum: bytes[5],
        };
        if pkt.calc_checksum() == pkt.checksum {
            Some(pkt)
        } else {
            None
        }
    }
}

pub mod radio;

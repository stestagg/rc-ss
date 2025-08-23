use embassy_futures::yield_now;
use esp_hal::peripherals::{IEEE802154, RADIO_CLK};
use esp_ieee802154::{Config, Error, Ieee802154};

use crate::ControlPacket;

/// Simple wrapper around the IEEE 802.15.4 driver for sending and receiving
/// [`ControlPacket`]s.
pub struct Radio<'a> {
    inner: Ieee802154<'a>,
}

impl<'a> Radio<'a> {
    /// Initialise the radio with default configuration and start receiving.
    pub fn new(radio: IEEE802154, radio_clocks: &mut RADIO_CLK) -> Self {
        let mut inner = Ieee802154::new(radio, radio_clocks);
        inner.set_config(Config::default());
        inner.start_receive();
        Self { inner }
    }

    /// Transmit a [`ControlPacket`].
    pub async fn send(&mut self, pkt: &ControlPacket) -> Result<(), Error> {
        let bytes = pkt.to_bytes();
        self.inner.transmit_raw(&bytes)?;
        Ok(())
    }

    /// Receive the next valid [`ControlPacket`].
    pub async fn receive(&mut self) -> ControlPacket {
        loop {
            if let Some(raw) = self.inner.get_raw_received() {
                let len = raw.data[0] as usize;
                if len >= ControlPacket::LEN {
                    let mut buf = [0u8; ControlPacket::LEN];
                    buf.copy_from_slice(&raw.data[1..1 + ControlPacket::LEN]);
                    if let Some(pkt) = ControlPacket::from_bytes(buf) {
                        return pkt;
                    }
                }
            }
            yield_now().await;
        }
    }
}


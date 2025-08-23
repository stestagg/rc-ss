#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use esp_hal::peripherals::Peripherals;
use panic_rtt_target as _;
use rc_core::{radio::Radio, ControlPacket};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("receiver boot");

    let mut peripherals = Peripherals::take();
    let ieee = peripherals.IEEE802154;
    let mut radio_clk = peripherals.RADIO_CLK;
    let mut radio = Radio::new(ieee, &mut radio_clk);

    loop {
        let pkt = radio.receive().await;
        info!("rx t={} s={} f={:?}", pkt.throttle, pkt.steering, pkt.flags);
    }
}

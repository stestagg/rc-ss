#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use esp_hal::peripherals::Peripherals;
use panic_rtt_target as _;
use rc_core::{radio::Radio, ControlPacket};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("transmitter boot");

    let mut peripherals = Peripherals::take();
    let ieee = peripherals.IEEE802154;
    let mut radio_clk = peripherals.RADIO_CLK;
    let mut radio = Radio::new(ieee, &mut radio_clk);

    let pkt = ControlPacket::FAILSAFE;

    loop {
        radio.send(&pkt).await.unwrap();
        yield_now().await;
    }
}

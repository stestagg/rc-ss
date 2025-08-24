#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use esp_alloc as _;
use panic_rtt_target as _;
use rc_core::{radio::Radio, ControlPacket};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("transmitter boot");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_alloc::heap_allocator!(size: 24 * 1024);
    let ieee = peripherals.IEEE802154;
    let mut radio = Radio::new(ieee);

    let pkt = ControlPacket::FAILSAFE;

    loop {
        radio.send(&pkt).await.unwrap();
        yield_now().await;
    }
}

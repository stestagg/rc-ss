#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use esp_hal::Config;
use esp_alloc as _;
use panic_rtt_target as _;
use rc_core::radio::Radio;

fn init_heap() {
    esp_alloc::heap_allocator!(size: 32 * 1024);
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("receiver boot");

    init_heap();
    let peripherals = esp_hal::init(Config::default());
    let ieee = peripherals.IEEE802154;
    let mut radio = Radio::new(ieee);

    loop {
        let pkt = radio.receive().await;
        info!("rx t={} s={} f={:?}", pkt.throttle, pkt.steering, pkt.flags);
    }
}

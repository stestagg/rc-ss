#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use panic_rtt_target as _;
use rc_core::ControlPacket;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("transmitter boot");
    let _ = ControlPacket {
        throttle: 0,
        steering: 0,
    };
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

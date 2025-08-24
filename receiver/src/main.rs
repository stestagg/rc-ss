#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use esp_alloc as _;
use panic_rtt_target as _;
use rc_core::{radio::Radio, ControlFlags, ControlPacket};
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig},
    time::Rate,
};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("receiver boot");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_alloc::heap_allocator!(size: 24 * 1024);
    let ieee = peripherals.IEEE802154;
    let mut radio = Radio::new(ieee);

    // PWM configuration for throttle and steering
    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(32)).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    let timer_cfg = clock_cfg
        .timer_clock_with_frequency(
            19_999,
            PwmWorkingMode::Increase,
            Rate::from_hz(ControlPacket::RATE_HZ),
        )
        .unwrap();
    mcpwm.timer0.start(timer_cfg);
    let (mut throttle, mut steering) = mcpwm.operator0.with_pins(
        peripherals.GPIO2,
        PwmPinConfig::UP_ACTIVE_HIGH,
        peripherals.GPIO3,
        PwmPinConfig::UP_ACTIVE_HIGH,
    );

    // Headlight control pin
    let mut headlight = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    // Initialise outputs to neutral
    let neutral = map_control(0);
    throttle.set_timestamp(neutral);
    steering.set_timestamp(neutral);

    loop {
        let pkt = radio.receive().await;
        info!("rx t={} s={} f={:?}", pkt.throttle, pkt.steering, pkt.flags);

        throttle.set_timestamp(map_control(pkt.throttle));
        steering.set_timestamp(map_control(pkt.steering));

        if pkt.flags.contains(ControlFlags::HEADLIGHT) {
            headlight.set_level(Level::High);
        } else {
            headlight.set_level(Level::Low);
        }
    }
}

fn map_control(val: i16) -> u16 {
    (1500 + (val as i32 * 500 / ControlPacket::THROTTLE_MAX as i32)) as u16
}

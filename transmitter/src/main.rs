#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use esp_alloc as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    gpio::{Input, InputConfig, Pull},
};
use panic_rtt_target as _;
use rc_core::{ControlFlags, ControlPacket, radio::Radio};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("transmitter boot");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_alloc::heap_allocator!(size: 24 * 1024);
    let ieee = peripherals.IEEE802154;
    let mut radio = Radio::new(ieee);

    // ADC configuration
    let adc1 = peripherals.ADC1;
    let mut adc_cfg = AdcConfig::new();
    let mut throttle_pin = adc_cfg.enable_pin(peripherals.GPIO2, Attenuation::_11dB);
    let mut steering_pin = adc_cfg.enable_pin(peripherals.GPIO3, Attenuation::_11dB);
    let mut adc = Adc::new(adc1, adc_cfg);

    // Button for auxiliary functions (active low)
    let headlight = Input::new(
        peripherals.GPIO4,
        InputConfig::default().with_pull(Pull::Up),
    );

    let mut ticker = Ticker::every(Duration::from_hz(ControlPacket::RATE_HZ as u64));

    loop {
        let throttle_raw: u16 = nb::block!(adc.read_oneshot(&mut throttle_pin)).unwrap();
        let steering_raw: u16 = nb::block!(adc.read_oneshot(&mut steering_pin)).unwrap();
        let (throttle, steering) = map_sticks(throttle_raw, steering_raw);
        let flags = read_flags(&headlight);

        let pkt = ControlPacket::new(throttle, steering, flags);
        radio.send(&pkt).await.unwrap();
        ticker.next().await;
    }
}

const THROTTLE_TRIM: i16 = 0;
const STEERING_TRIM: i16 = 0;

fn map_sticks(throttle_raw: u16, steering_raw: u16) -> (i16, i16) {
    let throttle = map_adc(
        throttle_raw,
        THROTTLE_TRIM,
        ControlPacket::THROTTLE_MIN,
        ControlPacket::THROTTLE_MAX,
    );
    let steering = map_adc(
        steering_raw,
        STEERING_TRIM,
        ControlPacket::STEERING_MIN,
        ControlPacket::STEERING_MAX,
    );

    (throttle, steering)
}

fn read_flags(headlight: &Input<'_>) -> ControlFlags {
    let mut flags = ControlFlags::empty();
    if headlight.is_low() {
        flags |= ControlFlags::HEADLIGHT;
    }
    flags
}

fn map_adc(raw: u16, trim: i16, min: i16, max: i16) -> i16 {
    let span = (max - min) as i32;
    let mut val = (raw as i32 * span) / 4095 + min as i32;
    val += trim as i32;
    val.clamp(min as i32, max as i32) as i16
}

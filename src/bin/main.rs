#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::ledc::timer::TimerIFace;
use panic_rtt_target as _;

use esp_hal::ledc::{channel, Ledc, LowSpeed, LSGlobalClkSource};
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer;
use esp_hal::time::Rate;
use esp_hal::gpio::{Output, Level, OutputConfig};
use esp_hal::peripherals::Peripherals;
use embedded_hal::pwm::SetDutyCycle;
use esp_hal::timer::timg::TimerGroup;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

enum Throttle{
    Neutral,
    Forward(u16),
    Backward(u16),
}

struct DutyCycle {
    max_cycle: f32
}

impl DutyCycle {
    fn new(max_cycle: u16) -> Self {
        DutyCycle { max_cycle: max_cycle as f32 }
    }

    fn for_throttle(&self, throttle: Throttle) -> u16 {
        let min_full = self.max_cycle * 1.0 / 20.0; // 1ms out of 20ms period = 5%
        let max_full = self.max_cycle * 2.0 / 20.0; // 2ms out of 20ms = 10%
        let neutral: f32 = (min_full + max_full) / 2.0; // 1.5ms out of 20ms = 7.5%
        let per_step = (max_full - neutral) / (u16::MAX as f32); // steps from neutral to max or min

        (match throttle {
            Throttle::Neutral => neutral,
            Throttle::Forward(value) => neutral + per_step * value as f32,
            Throttle::Backward(value) => neutral - per_step * value as f32,
        }) as u16
    }
}


#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let mut led = Output::new(peripherals.GPIO14, Level::Low, OutputConfig::default());

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0.configure(timer::config::Config {
        duty: timer::config::Duty::Duty13Bit, // gives 8192 steps
        clock_source: timer::LSClockSource::APBClk,
        frequency: Rate::from_hz(50), // 50 Hz = 20 ms period
    }).unwrap();


    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    channel0.configure(channel::config::Config {
        timer: &lstimer0,
        duty_pct: 10,
        pin_config: channel::config::PinConfig::PushPull,
    }).unwrap();

    let max_duty = channel0.max_duty_cycle(); // e.g. 8191 if 13-bit
    let cycles = DutyCycle::new(max_duty);

    channel0.set_duty_cycle(cycles.for_throttle(Throttle::Neutral));

    info!("Waiting for 1 seconds before starting");

    // Sleep for 1 seconds to allow the system to stabilize
    Timer::after(Duration::from_secs(1)).await;

    info!("Setting throttle to minimum");

    let mut i = 0;
    while i < u16::MAX {
        i = i.saturating_add(500);
        let duty = cycles.for_throttle(Throttle::Forward(i));
        channel0.set_duty_cycle(duty);
        info!("Setting throttle to {} ({})", i, duty);
        Timer::after(Duration::from_millis(500)).await;
    }
    
    // channel0.set_duty_cycle(cycles.for_throttle(Throttle::Neutral));

    loop {
        Timer::after(Duration::from_secs(300)).await;
    }
    
}

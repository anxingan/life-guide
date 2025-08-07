use defmt::info;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_hal::{
    gpio::Output,
    ledc::{
        channel::{self, ChannelIFace},
        LowSpeed,
    },
    system::Cpu,
};

#[embassy_executor::task]
// LED闪烁
pub async fn blink(mut led: Output<'static>) {
    info!("Starting blink() on core {}", Cpu::current() as usize);
    loop {
        led.toggle();
        if led.is_set_low() {
            info!("LED off");
        } else {
            info!("LED on");
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
// LED接收信号更改状态
pub async fn led_signal(
    mut led: Output<'static>,
    signal: &'static Signal<CriticalSectionRawMutex, bool>,
) {
    info!("Starting led_signal() on core {}", Cpu::current() as usize);
    loop {
        let _state = signal.wait().await;
        led.toggle();
        if led.is_set_low() {
            info!("LED off");
        } else {
            info!("LED on");
        }
    }
}

#[embassy_executor::task]
// LED呼吸灯
pub async fn led_fader(channel0: &'static channel::Channel<'static, LowSpeed>) {
    info!("Starting led_fader() on core {}", Cpu::current() as usize);
    loop {
        channel0.start_duty_fade(0, 100, 1000).unwrap();
        while channel0.is_duty_fade_running() {}
        channel0.start_duty_fade(100, 0, 1000).unwrap();
        while channel0.is_duty_fade_running() {}
    }
}

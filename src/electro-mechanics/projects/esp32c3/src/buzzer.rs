use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{gpio::Output, system::Cpu};

#[embassy_executor::task]
// 控制有源蜂鸣器，每500毫秒切换一次状态。
pub async fn buzzer_active(mut buzzer: Output<'static>) {
    info!(
        "Starting buzzer_active() on core {}",
        Cpu::current() as usize
    );
    loop {
        buzzer.toggle();
        if buzzer.is_set_low() {
            info!("Buzzer off");
        } else {
            info!("Buzzer on");
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

use defmt::info;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use esp_hal::{gpio::Input, system::Cpu};

#[embassy_executor::task]
// 按钮发送信号
pub async fn button_signal(
    mut button: Input<'static>,
    signal: &'static Signal<CriticalSectionRawMutex, bool>,
) {
    info!("Starting button_signal on core {}", Cpu::current() as usize);
    loop {
        // 等待按钮按下事件
        button.wait_for_falling_edge().await;
        info!("Button pressed, send signal");
        signal.signal(true);
    }
}

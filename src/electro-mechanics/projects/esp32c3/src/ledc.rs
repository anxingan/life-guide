use defmt::info;
use esp_hal::{
    gpio::Output,
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    system::Cpu,
    time::Rate,
};

#[embassy_executor::task]
// LEDC(LED Controller)异步任务函数，用于控制LED的PWM输出
// 参数：
// - ledc: LED控制器实例
// - led: 作为PWM输出的LED引脚
pub async fn ledc_led(mut ledc: Ledc<'static>, output_pin: Output<'static>) {
    info!("Starting ledc_led() on core {}", Cpu::current() as usize);
    // 设置LEDC全局慢时钟源为APB时钟
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    // 创建低速定时器0实例
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    // 配置定时器参数
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .expect("failed to configure ledc timer");
    // 创建通道0，绑定到指定的LED引脚
    let mut chan0 = ledc.channel(channel::Number::Channel0, output_pin);
    // 配置通道参数
    chan0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .expect("failed to configure ledc channel");
    loop {
        // 从0%占空比渐变到100%，耗时1000毫秒
        chan0.start_duty_fade(0, 100, 1000).unwrap();
        // 等待渐变完成
        while chan0.is_duty_fade_running() {}
        // 从100%占空比渐变到0%，耗时1000毫秒
        chan0.start_duty_fade(100, 0, 1000).unwrap();
        // 等待渐变完成
        while chan0.is_duty_fade_running() {}
    }
}

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::{
    fmt::Write,
    sync::atomic::{self, AtomicU32},
};

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    dma::NoDma,
    exti::ExtiInput,
    gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed},
    usart::{Config, UartTx},
};
use embassy_time::{Duration, Timer};
use heapless::String;
use panic_probe as _;

static BLINK_MS: AtomicU32 = AtomicU32::new(2000);

#[embassy_executor::task]
async fn led_task(led: AnyPin) {
    let mut led = Output::new(led, Level::Low, Speed::Low);
    loop {
        let delay = BLINK_MS.load(atomic::Ordering::Relaxed);
        led.toggle();
        info!(
            "Led turned {}",
            if led.is_set_high() { "on" } else { "off" }
        );
        Timer::after(Duration::from_millis(delay.into())).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let button = Input::new(p.PA0, Pull::None);
    let mut button = ExtiInput::new(button, p.EXTI0);

    let mut delay = BLINK_MS.load(atomic::Ordering::Relaxed);

    let mut usart = UartTx::new(p.USART1, p.PC4, NoDma, Config::default()).unwrap();

    spawner.spawn(led_task(p.PE8.degrade())).unwrap();

    let mut value = 0u8;
    let mut msg: String<8> = String::new();

    loop {
        button.wait_for_rising_edge().await;
        delay -= 200;
        info!("Delay set to {}ms", delay);
        if delay < 200 {
            delay = 2000;
            info!("Delay reset to {}ms", delay);
        }
        BLINK_MS.store(delay, atomic::Ordering::Relaxed);

        core::writeln!(&mut msg, "{:02}", value).unwrap();

        usart.blocking_write(msg.as_bytes()).unwrap();

        value = value.wrapping_add(1);

        msg.clear();
    }
}

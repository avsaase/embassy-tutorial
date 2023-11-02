#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    peripherals::PE8,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, PE8>, interval: Duration) {
    loop {
        led.set_high();
        info!("High");
        Timer::after(interval).await;
        info!("Low");
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let led = Output::new(p.PE8, Level::Low, Speed::Medium);
    spawner
        .spawn(blinker(led, Duration::from_millis(250)))
        .unwrap();
}

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    i2c::{self, I2c},
    peripherals,
    time::khz,
    usart::UartTx,
};
use embassy_time::Timer;
use embedded_hal::i2c::I2c as eI2c;
use heapless::String;
use panic_probe as _;

const LM75_ADDRESS: u8 = 0b_100_1000;
const TEMP_REG: u8 = 0b_0000_0000;
const PRODUCT_ID_REG: u8 = 0b_0000_0111;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::InterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        NoDma,
        NoDma,
        khz(100),
        Default::default(),
    );

    let mut usart = UartTx::new(p.USART1, p.PC4, NoDma, Default::default()).unwrap();

    let mut msg: String<64> = String::new();

    let mut rx_buffer: [u8; 2] = [0; 2];

    loop {
        i2c.blocking_write_read(LM75_ADDRESS, &[TEMP_REG], &mut rx_buffer)
            .unwrap();
        let temp_bits = (((rx_buffer[0] as u16) << 8) | rx_buffer[1] as u16) >> 7;
        let temp = decode_temperature(temp_bits);
        info!("Temperature: {} celsius", temp);

        core::writeln!(&mut msg, "Temperature: {} celsius", temp).unwrap();
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();

        Timer::after_millis(1000).await;
    }
}

fn decode_temperature(temp_data: u16) -> f32 {
    let sign_bit = (temp_data & 0x100) >> 8;
    let temp_bits = temp_data & 0xFF;

    let temperature_celsius = if sign_bit == 0 {
        f32::from(temp_bits) * 0.5
    } else {
        -f32::from(256 - temp_bits) * 0.5
    };
    temperature_celsius
}

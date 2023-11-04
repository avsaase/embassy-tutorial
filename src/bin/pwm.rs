#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::OutputType,
    peripherals::{PA9, TIM1},
    time::hz,
    timer::{
        simple_pwm::{PwmPin, SimplePwm},
        Channel, CountingMode,
    },
};
use embassy_time::Timer;
use panic_probe as _;

const SERVO_FREQUENCY_HZ: u32 = 50;
const SLEW_RATE: u32 = 500;
const DELAY_MS: u32 = 20;

#[embassy_executor::task]
async fn pwm_task(pin: PA9, timer: TIM1) {
    let pwm_pin = PwmPin::new_ch2(pin, OutputType::PushPull);
    let mut pwm = SimplePwm::new(
        timer,
        None,
        Some(pwm_pin),
        None,
        None,
        hz(SERVO_FREQUENCY_HZ),
        CountingMode::EdgeAlignedUp,
    );

    let max_duty = pwm.get_max_duty();
    let max_duty_per_sec = f32::from(max_duty) * SERVO_FREQUENCY_HZ as f32;
    pwm.enable(Channel::Ch2);

    let mut duty_cycle_us = 1000_u32;
    let mut change = Change::Increasing;

    loop {
        let duty = (duty_cycle_us as f32 / 1_000_000. * max_duty_per_sec) as u16;
        pwm.set_duty(Channel::Ch2, duty);
        info!("PWM set to {}us. Duty set to {}", duty_cycle_us, duty);

        duty_cycle_us = match change {
            Change::Increasing => duty_cycle_us + SLEW_RATE * DELAY_MS / 1000,
            Change::Decreasing => duty_cycle_us - SLEW_RATE * DELAY_MS / 1000,
        };
        if duty_cycle_us > 2000 {
            change = Change::Decreasing;
        } else if duty_cycle_us < 1000 {
            change = Change::Increasing;
        }

        Timer::after_millis(DELAY_MS as u64).await;
    }
}

enum Change {
    Increasing,
    Decreasing,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    spawner.spawn(pwm_task(p.PA9, p.TIM1)).unwrap();
}

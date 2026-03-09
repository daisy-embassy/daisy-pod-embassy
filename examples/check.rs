#![no_std]
#![no_main]

//! A test example to verify pin and peripheral functionality

use daisy_embassy::hal::bind_interrupts;
use daisy_embassy::hal::gpio::Pull;
use daisy_embassy::hal::mode::Async;
use daisy_embassy::hal::{self, exti::ExtiInput};
use daisy_embassy::new_daisy_board;
use daisy_pod_embassy::peri::DaisyPodPeripherals;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};
bind_interrupts!(
    pub struct Irqs{
        EXTI9_5 => hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI9_5>;
        EXTI2 => hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let rcc = daisy_embassy::default_rcc();
    let p = hal::init(rcc);
    info!("Hello World!");
    let daisy_p = new_daisy_board!(p);
    let pod_p = DaisyPodPeripherals::new(daisy_p, p.ADC1, p.ADC2, p.USART1, p.USB_OTG_HS);
    let button = ExtiInput::new(pod_p.tact_switches.tac_switch_1, p.EXTI9, Pull::Down, Irqs);
    spawner.must_spawn(tac_switch_task(button, "tac_1"));
    let button = ExtiInput::new(pod_p.tact_switches.tac_switch_2, p.EXTI2, Pull::Down, Irqs);
    spawner.must_spawn(tac_switch_task(button, "tac_2"));
    let mut led = pod_p.user_led;

    loop {
        info!("on");
        led.on();
        Timer::after_millis(300).await;

        info!("off");
        led.off();
        Timer::after_millis(300).await;
    }
}

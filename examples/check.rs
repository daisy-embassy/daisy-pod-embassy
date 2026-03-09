#![no_std]
#![no_main]

//! A test example to verify pin and peripheral functionality

use daisy_embassy::hal::bind_interrupts;
use daisy_embassy::hal::gpio::Pull;
use daisy_embassy::hal::mode::Async;
use daisy_embassy::hal::{self, exti::ExtiInput};
use daisy_embassy::led::UserLed;
use daisy_embassy::new_daisy_board;
use daisy_pod_embassy::peri::DaisyPodPeripherals;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use grounded::uninit::GroundedArrayCell;

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

    spawner.must_spawn(blinky(pod_p.user_led));

    let button = ExtiInput::new(pod_p.tact_switches.tac_switch_1, p.EXTI9, Pull::Down, Irqs);
    spawner.must_spawn(tac_switch_task(button, "tac_1"));
    let button = ExtiInput::new(pod_p.tact_switches.tac_switch_2, p.EXTI2, Pull::Down, Irqs);
    spawner.must_spawn(tac_switch_task(button, "tac_2"));
}

#[embassy_executor::task]
async fn blinky(mut pin: UserLed<'static>) {
    let mut ticker = Ticker::every(Duration::from_millis(300));
    loop {
        pin.off();
        ticker.next().await;
        pin.on();
        ticker.next().await;
    }
}
#[embassy_executor::task(pool_size = 2)]
async fn tac_switch_task(mut pin: ExtiInput<'static, Async>, identifier: &'static str) -> ! {
    loop {
        pin.wait_for_low().await;
        info!("{} pressed", identifier);
    }
}

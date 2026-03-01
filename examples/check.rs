#![no_std]
#![no_main]

//! A test example to verify pin and peripheral functionality

use daisy_embassy::hal;
use daisy_embassy::new_daisy_board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default());
    info!("Hello World!");
    let daisy_p = new_daisy_board!(p);
    let mut led = daisy_p.user_led;

    loop {
        info!("on");
        led.on();
        Timer::after_millis(300).await;

        info!("off");
        led.off();
        Timer::after_millis(300).await;
    }
}

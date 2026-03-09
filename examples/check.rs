#![no_std]
#![no_main]

//! A test example to verify pin and peripheral functionality

use daisy_embassy::hal::adc::{AdcChannel as _, SampleTime};
use daisy_embassy::hal::gpio::Pull;
use daisy_embassy::hal::mode::Async;
use daisy_embassy::hal::peripherals::{DMA2_CH0, DMA2_CH1};
use daisy_embassy::hal::{self, exti::ExtiInput};
use daisy_embassy::hal::{Peri, bind_interrupts, dma, peripherals};
use daisy_embassy::led::UserLed;
use daisy_embassy::new_daisy_board;
use daisy_pod_embassy::peri::{DaisyPodPeripherals, Pot1, Pot2};
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker, Timer};
use grounded::uninit::GroundedArrayCell;

use {defmt_rtt as _, panic_probe as _};

#[unsafe(link_section = ".sram1_bss")]
static ADC_BUFFER: GroundedArrayCell<u16, 2> = GroundedArrayCell::uninit();

bind_interrupts!(
    pub struct Irqs{
        EXTI9_5 => hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI9_5>;
        EXTI2 => hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI2>;
        DMA2_STREAM0 => dma::InterruptHandler<peripherals::DMA2_CH0>;
        DMA2_STREAM1 => dma::InterruptHandler<peripherals::DMA2_CH1>;
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
    spawner.must_spawn(pot1_task(pod_p.pot1, p.DMA2_CH0));
    spawner.must_spawn(pot2_task(pod_p.pot2, p.DMA2_CH1));
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
    let mut debounce = Ticker::every(Duration::from_millis(100));
    loop {
        pin.wait_for_low().await;
        info!("{} pressed", identifier);
        debounce.next().await;
    }
}

#[embassy_executor::task]
async fn pot1_task(mut pot: Pot1<'static>, mut dma: Peri<'static, DMA2_CH0>) {
    let read_buffer = unsafe {
        ADC_BUFFER.initialize_all_copied(0);
        let (ptr, len) = ADC_BUFFER.get_ptr_len();
        core::slice::from_raw_parts_mut(ptr, len)
    };
    let mut vrefint_channel = pot.adc.enable_vrefint().degrade_adc();
    let mut pin = pot.pin.degrade_adc();
    loop {
        pot.adc
            .read(
                dma.reborrow(),
                Irqs,
                [
                    (&mut vrefint_channel, SampleTime::CYCLES387_5),
                    (&mut pin, SampleTime::CYCLES810_5),
                ]
                .into_iter(),
                read_buffer,
            )
            .await;
        let vrefint = read_buffer[0];
        let measured = read_buffer[1];
        info!("pot1 vrefint: {}", vrefint);
        info!("pot1 measured: {}", measured);
        Timer::after_millis(500).await;
    }
}
#[embassy_executor::task]
async fn pot2_task(mut pot: Pot2<'static>, mut dma: Peri<'static, DMA2_CH1>) {
    let read_buffer = unsafe {
        ADC_BUFFER.initialize_all_copied(0);
        let (ptr, len) = ADC_BUFFER.get_ptr_len();
        core::slice::from_raw_parts_mut(ptr, len)
    };
    let mut vrefint_channel = pot.adc.enable_vrefint().degrade_adc();
    let mut pin = pot.pin.degrade_adc();
    loop {
        pot.adc
            .read(
                dma.reborrow(),
                Irqs,
                [
                    (&mut vrefint_channel, SampleTime::CYCLES387_5),
                    (&mut pin, SampleTime::CYCLES810_5),
                ]
                .into_iter(),
                read_buffer,
            )
            .await;
        let vrefint = read_buffer[0];
        let measured = read_buffer[1];
        info!("pot2 vrefint: {}", vrefint);
        info!("pot2 measured: {}", measured);
        Timer::after_millis(500).await;
    }
}

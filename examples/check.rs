#![no_std]
#![no_main]

//! A test example to verify pin and peripheral functionality

use daisy_embassy::hal::adc::{AdcChannel as _, SampleTime};
use daisy_embassy::hal::gpio::{Input, Level, Output, Pull, Speed};
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
        EXTI15_10 => hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI15_10>;
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

    let enc = pod_p.rotary_encoder;
    let enc_a = ExtiInput::new(enc.enc_a, p.EXTI11, Pull::Up, Irqs);
    let enc_b = Input::new(enc.enc_b, Pull::Up);
    let enc_click = ExtiInput::new(enc.enc_click, p.EXTI6, Pull::Up, Irqs);
    spawner.must_spawn(rotary_encoder_task(enc_a, enc_b, enc_click));

    // RGB LED1: 500ms per step
    let rgb1 = pod_p.rgb_led1;
    spawner.must_spawn(rgb_led_task(
        Output::new(rgb1.r, Level::Low, Speed::Low),
        Output::new(rgb1.g, Level::Low, Speed::Low),
        Output::new(rgb1.b, Level::Low, Speed::Low),
        "rgb_led1",
        Duration::from_micros(500),
    ));
    // RGB LED2: 700ms per step (offset timing so they differ visually)
    let rgb2 = pod_p.rgb_led2;
    spawner.must_spawn(rgb_led_task(
        Output::new(rgb2.r, Level::Low, Speed::Low),
        Output::new(rgb2.g, Level::Low, Speed::Low),
        Output::new(rgb2.b, Level::Low, Speed::Low),
        "rgb_led2",
        Duration::from_millis(700),
    ));
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

/// Cycles through all 8 RGB on/off combinations to verify each LED colour works.
/// The `interval_ms` argument controls how long each combination is held, allowing
/// LED1 and LED2 to be driven at different speeds so they are visually distinct.
///
/// Combination order (R G B):
///   0 = 000 (all off)  1 = 001 (B)   2 = 010 (G)   3 = 011 (cyan)
///   4 = 100 (R)        5 = 101 (mag) 6 = 110 (yel)  7 = 111 (white)
#[embassy_executor::task(pool_size = 2)]
async fn rgb_led_task(
    mut r: Output<'static>,
    mut g: Output<'static>,
    mut b: Output<'static>,
    label: &'static str,
    interval: Duration,
) {
    const COMBOS: [(bool, bool, bool); 8] = [
        (false, false, false), // off
        (false, false, true),  // B   – blue
        (false, true, false),  // G   – green
        (false, true, true),   // G+B – cyan
        (true, false, false),  // R   – red
        (true, false, true),   // R+B – magenta
        (true, true, false),   // R+G – yellow
        (true, true, true),    // all – white
    ];
    let mut idx: usize = 0;
    loop {
        let (r_on, g_on, b_on) = COMBOS[idx];
        r.set_level(if r_on { Level::High } else { Level::Low });
        g.set_level(if g_on { Level::High } else { Level::Low });
        b.set_level(if b_on { Level::High } else { Level::Low });
        info!("{} combo {}: R={} G={} B={}", label, idx, r_on, g_on, b_on);
        idx = (idx + 1) % COMBOS.len();
        Timer::after(interval).await;
    }
}

/// Detects rotary encoder rotation direction and button press.
///
/// Direction is decoded by sampling enc_b on every enc_a edge:
///   enc_a != enc_b  →  clockwise
///   enc_a == enc_b  →  counter-clockwise
#[embassy_executor::task]
async fn rotary_encoder_task(
    mut enc_a: ExtiInput<'static, Async>,
    enc_b: Input<'static>,
    mut enc_click: ExtiInput<'static, Async>,
) {
    loop {
        // Wait for either an enc_a edge or a button press.
        // Both futures are polled; whichever resolves first is handled.
        let a_edge = enc_a.wait_for_any_edge();
        let click = enc_click.wait_for_falling_edge();
        match embassy_futures::select::select(a_edge, click).await {
            embassy_futures::select::Either::First(()) => {
                let a = enc_a.is_high();
                let b = enc_b.is_high();
                if a != b {
                    info!("rotary encoder: CW (clockwise)");
                } else {
                    info!("rotary encoder: CCW (counter-clockwise)");
                }
            }
            embassy_futures::select::Either::Second(()) => {
                info!("rotary encoder: click");
            }
        }
    }
}

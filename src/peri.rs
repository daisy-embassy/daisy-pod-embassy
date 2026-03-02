//! https://daisy.audio/product/Daisy-Pod/#pinout

use daisy_embassy::audio::AudioPeripherals;
use daisy_embassy::flash::FlashBuilder;
use daisy_embassy::hal::Peri;
use daisy_embassy::hal::adc::Adc;
use daisy_embassy::hal::peripherals::{ADC1, ADC2, USART1, USB_OTG_HS};
use daisy_embassy::led::UserLed;
use daisy_embassy::pins::*;
use daisy_embassy::sdram::SdRamBuilder;
use daisy_embassy::usb::UsbPeripherals;

pub struct DaisyPodPeripherals<'a> {
    pub tact_switches: TactSwitches<'a>,
    pub rgb_led1: RGBLed1<'a>,
    pub rgb_led2: RGBLed2<'a>,
    pub pot1: Pot1<'a>,
    pub pot2: Pot2<'a>,
    pub rotary_encoder: RotaryEncoder<'a>,
    pub midi_jack: MidiJack<'a>,
    pub usb_peri: UsbPeri<'a>,
    pub expansion_pins: ExpansionPins<'a>,
    pub user_led: UserLed<'a>,
    pub audio_peripherals: AudioPeripherals<'a>,
    pub flash: FlashBuilder<'a>,
    pub sdram: SdRamBuilder<'a>,
    pub usb_peripherals: UsbPeripherals<'a>,
    // on board "BOOT" button.
    pub boot: Boot<'a>,
}

impl<'a> DaisyPodPeripherals<'a> {
    pub fn new(
        board: daisy_embassy::DaisyBoard<'a>,
        adc1: Peri<'a, ADC1>,
        adc2: Peri<'a, ADC2>,
        usart: Peri<'a, USART1>,
        usp_peri: Peri<'a, USB_OTG_HS>,
    ) -> Self {
        let p = board.pins;
        Self {
            tact_switches: TactSwitches {
                tac_switch_1: p.d27,
                tac_switch_2: p.d28,
            },
            rgb_led1: RGBLed1 {
                r: p.d20,
                g: p.d19,
                b: p.d18,
            },
            rgb_led2: RGBLed2 {
                r: p.d17,
                g: p.d24,
                b: p.d23,
            },
            pot1: Pot1 {
                pin: p.d21,
                adc: Adc::new(adc1),
            },
            pot2: Pot2 {
                pin: p.d15,
                adc: Adc::new(adc2),
            },
            rotary_encoder: RotaryEncoder {
                enc_a: p.d26,
                enc_b: p.d25,
                enc_click: p.d13,
            },
            midi_jack: MidiJack { pin: p.d14, usart },
            usb_peri: UsbPeri {
                usb_id: p.d0,
                usb_d_plus: p.d30,
                usb_d_minus: p.d29,
                usb_peri: usp_peri,
            },
            expansion_pins: ExpansionPins {
                d7: p.d7,
                d8: p.d8,
                d9: p.d9,
                d10: p.d10,
                d11: p.d11,
                d12: p.d12,
                d16: p.d16,
                d22: p.d22,
            },
            user_led: board.user_led,
            audio_peripherals: board.audio_peripherals,
            flash: board.flash,
            sdram: board.sdram,
            usb_peripherals: board.usb_peripherals,
            boot: board.boot,
        }
    }
}

pub struct TactSwitches<'a> {
    pub tac_switch_1: SeedPin27<'a>,
    pub tac_switch_2: SeedPin28<'a>,
}

pub struct RGBLed1<'a> {
    pub r: SeedPin20<'a>,
    pub g: SeedPin19<'a>,
    pub b: SeedPin18<'a>,
}

pub struct RGBLed2<'a> {
    pub r: SeedPin17<'a>,
    pub g: SeedPin24<'a>,
    pub b: SeedPin23<'a>,
}

pub struct Pot1<'a> {
    pub pin: SeedPin21<'a>,
    pub adc: Adc<'a, ADC1>,
}

pub struct Pot2<'a> {
    pub pin: SeedPin15<'a>,
    pub adc: Adc<'a, ADC2>,
}

pub struct RotaryEncoder<'a> {
    pub enc_a: SeedPin26<'a>,
    pub enc_b: SeedPin25<'a>,
    pub enc_click: SeedPin13<'a>,
}

pub struct MidiJack<'a> {
    pub pin: SeedPin14<'a>,
    pub usart: Peri<'a, USART1>,
}

pub struct UsbPeri<'a> {
    pub usb_id: SeedPin0<'a>,
    pub usb_d_plus: SeedPin30<'a>,
    pub usb_d_minus: SeedPin29<'a>,
    pub usb_peri: Peri<'a, USB_OTG_HS>,
}

pub struct ExpansionPins<'a> {
    pub d7: SeedPin7<'a>,
    pub d8: SeedPin8<'a>,
    pub d9: SeedPin9<'a>,
    pub d10: SeedPin10<'a>,
    pub d11: SeedPin11<'a>,
    pub d12: SeedPin12<'a>,
    pub d16: SeedPin16<'a>,
    pub d22: SeedPin22<'a>,
}

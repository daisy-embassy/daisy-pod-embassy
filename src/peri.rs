//! https://daisy.audio/product/Daisy-Pod/#pinout

use daisy_embassy::hal::adc::Adc;
use daisy_embassy::hal::peripherals::{ADC1, ADC2, USART1, USB_OTG_HS};
use daisy_embassy::pins::*;

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
    pub usart: USART1,
}

pub struct UsbPeri<'a> {
    pub usb_id: SeedPin0<'a>,
    pub usb_d_plus: SeedPin30<'a>,
    pub usb_d_minus: SeedPin29<'a>,
    pub usb_peri: USB_OTG_HS,
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

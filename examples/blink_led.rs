#![no_std]
#![no_main]

pub extern crate stm32f7xx_hal as hal;

mod led;

use cortex_m_rt::entry;
use hal::{delay::Delay, prelude::*};
use panic_halt as _;

use led::Leds;

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let peripherals = hal::device::Peripherals::take().unwrap();

    // reset and clock control
    let rcc = peripherals.RCC.constrain();

    // clock configuration using the default settings (all clocks run at 8 MHz)
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc.cfgr.sysclk(16.mhz()).freeze();
    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    let mut leds = Leds::new(peripherals.GPIOG.split(), peripherals.GPIOE.split());

    // leds.
    loop {
        for i in 0..4 {
            leds[i].on();
            delay.delay_ms(100u16);
        }
        for i in 0..4 {
            leds[i].off();
            delay.delay_ms(100u16);
        }
    }
}

#![no_std]
#![no_main]

pub extern crate stm32f7xx_hal as hal;

mod led;

use cortex_m_rt::entry;
use hal::{delay::Delay, prelude::*, serial::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let peripherals = hal::device::Peripherals::take().unwrap();

    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    let gpioc = peripherals.GPIOC.split();
    let tx = gpioc.pc6.into_alternate_af8();
    let rx = gpioc.pc7.into_alternate_af8();

    let serial = Serial::new(
        peripherals.USART6,
        (tx, rx),
        clocks,
        hal::serial::Config {
            baud_rate: 115_200.bps(),
            oversampling: hal::serial::Oversampling::By16,
        },
    );

    let (mut tx, _) = serial.split();

    let hello: &str = "Hello, I'm a STM32F7xx!\r\n";
    loop {
        hello
            .as_bytes()
            .into_iter()
            .try_for_each(|c| nb::block!(tx.write(*c))).unwrap();
        delay.delay_ms(500_u16);
    }
}

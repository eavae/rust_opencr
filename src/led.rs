use core::ops;

use embedded_hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
use hal::gpio::gpioe::PE;
use hal::gpio::gpiog::PG;
use hal::gpio::{Output, PushPull};
use hal::prelude::*;

pub trait Switch {
    fn on(&mut self) {}

    fn off(&mut self) {}
}

pub struct Led<T>(T);

impl Switch for Led<PE<Output<PushPull>>> {
    fn on(&mut self) {
        self.0.set_high();
    }

    fn off(&mut self) {
        self.0.set_low();
    }
}
impl Switch for Led<PG<Output<PushPull>>> {
    fn on(&mut self) {
        self.0.set_high();
    }

    fn off(&mut self) {
        self.0.set_low();
    }
}

// impl Led<PE<Output<PushPull>>> {
//     pub fn on(&mut self) {
//         self.0.set_high();
//     }

//     pub fn off(&mut self) {
//         self.0.set_low();
//     }
// }

// impl Led<PG<Output<PushPull>>> {
//     pub fn on(&mut self) {
//         self.0.set_high();
//     }

//     pub fn off(&mut self) {
//         self.0.set_low();
//     }
// }

impl Into<Led<PE<Output<PushPull>>>> for PE<Output<PushPull>> {
    fn into(self) -> Led<PE<Output<PushPull>>> {
        Led(self)
    }
}

impl Into<Led<PG<Output<PushPull>>>> for PG<Output<PushPull>> {
    fn into(self) -> Led<PG<Output<PushPull>>> {
        Led(self)
    }
}

/// USER1, USER2, USER3, USER4
pub struct Leds(
    Led<PG<Output<PushPull>>>,
    Led<PE<Output<PushPull>>>,
    Led<PE<Output<PushPull>>>,
    Led<PG<Output<PushPull>>>,
);

macro_rules! init_led {
    ($gpiox:ident, $pxn:ident) => {
        $gpiox.$pxn.into_push_pull_output().downgrade().into()
    };
}

// {GPIOG, GPIO_PIN_12,  NULL,     NO_ADC        , NULL   ,   NO_PWM       , NO_EXTI },  // 22 BDPIN_LED_USER_1
// {GPIOE, GPIO_PIN_5,   NULL,     NO_ADC        , NULL   ,   NO_PWM       , NO_EXTI },  // 23 BDPIN_LED_USER_2
// {GPIOE, GPIO_PIN_4,   NULL,     NO_ADC        , NULL   ,   NO_PWM       , NO_EXTI },  // 24 BDPIN_LED_USER_3
// {GPIOG, GPIO_PIN_10,  NULL,     NO_ADC        , NULL   ,   NO_PWM       , NO_EXTI },  // 25 BDPIN_LED_USER_4

impl Leds {
    pub fn new(gpiog: hal::gpio::gpiog::Parts, gpioe: hal::gpio::gpioe::Parts) -> Self {
        // gpioe.pe12.into_push_pull_output().downgrade().into();
        // gpiog.pg12.into_push_pull_output().downgrade().into();
        Leds(
            init_led!(gpiog, pg12),
            init_led!(gpioe, pe5),
            init_led!(gpioe, pe4),
            init_led!(gpiog, pg10),
        )
    }
}

impl ops::Index<usize> for Leds {
    type Output = dyn Switch;
    
    fn index(&self, d: usize) -> &(dyn Switch + 'static) {
        match d {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!()
        }
    }
}

impl ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, d: usize) -> &mut (dyn Switch + 'static) {
        match d {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!()
        }
    }
}

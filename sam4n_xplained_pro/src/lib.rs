#![no_std]

pub use atsam4_hal as hal;
use paste::paste;

use atsam4_hal::{define_pin_map, gpio::*};

define_pin_map! {
    struct Pins,

    // Onboard LED
    pin led0 = b14<Output<OpenDrain>, into_open_drain_output>,

    // Serial Console (UART0)
    pin uart0_rx = a9<PfA, into_peripheral_function_a>,
    pin uart0_tx = a10<PfA, into_peripheral_function_a>,
}

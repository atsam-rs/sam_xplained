#![no_std]

pub use atsam4_hal as hal;
use paste::paste;

use atsam4_hal::{define_pin_map, gpio::*, pac::MATRIX};

define_pin_map! {
    struct Pins,

    // Onboard LED
    pin led0 = d22<Output<OpenDrain>, into_open_drain_output>,

    // Onboard Button labeled SW0
    pin sw0 = a2<Input<PullUp>, into_pull_up_input>,

    // Serial Console (UART0)
    pin uart0_rx = a9<PfA, into_peripheral_function_a>,
    pin uart0_tx = a10<PfA, into_peripheral_function_a>,

    // Static Memory Controller Pins
    pin ncs1 = d18<PfA, into_peripheral_function_a>,
    pin ncs3 = d19<PfA, into_peripheral_function_a>,

    pin nrd = c11<PfA, into_peripheral_function_a>,
    pin nwe = c8<PfA, into_peripheral_function_a>,

    pin d0 = c0<PfA, into_peripheral_function_a>,
    pin d1 = c1<PfA, into_peripheral_function_a>,
    pin d2 = c2<PfA, into_peripheral_function_a>,
    pin d3 = c3<PfA, into_peripheral_function_a>,
    pin d4 = c4<PfA, into_peripheral_function_a>,
    pin d5 = c5<PfA, into_peripheral_function_a>,
    pin d6 = c6<PfA, into_peripheral_function_a>,
    pin d7 = c7<PfA, into_peripheral_function_a>,

    pin a0 = c18<PfA, into_peripheral_function_a>,
    pin a1 = c19<PfA, into_peripheral_function_a>,
    pin a2 = c20<PfA, into_peripheral_function_a>,
    pin a3 = c21<PfA, into_peripheral_function_a>,
    pin a4 = c22<PfA, into_peripheral_function_a>,
    pin a5 = c23<PfA, into_peripheral_function_a>,
    pin a6 = c24<PfA, into_peripheral_function_a>,
    pin a7 = c25<PfA, into_peripheral_function_a>,
    pin a8 = c26<PfA, into_peripheral_function_a>,
    pin a9 = c27<PfA, into_peripheral_function_a>,

    pin a10 = c28<PfA, into_peripheral_function_a>,
    pin a11 = c29<PfA, into_peripheral_function_a>,
    pin a12 = c30<PfA, into_peripheral_function_a>,
    pin a13 = c31<PfA, into_peripheral_function_a>,

    pin a14 = a18<PfC, into_peripheral_function_c>,
    pin a15 = a19<PfC, into_peripheral_function_c>,
    pin a16 = a20<PfC, into_peripheral_function_c>,

    pin a17 = a0<PfC, into_peripheral_function_c>,
    pin a18 = a1<PfC, into_peripheral_function_c>,

    pin a19 = a23<PfC, into_peripheral_function_c>,
    pin a20 = a24<PfC, into_peripheral_function_c>,

    pin a21 = c16<PfA, into_peripheral_function_a>,
    pin a22 = c17<PfA, into_peripheral_function_a>,

    pin a23 = a25<PfC, into_peripheral_function_c>,

    // Ethernet MAC (GMAC)
    pin grefck  = d0<PfA, into_peripheral_function_a>,
    pin gtxen   = d1<PfA, into_peripheral_function_a>,
    pin gtx0    = d2<PfA, into_peripheral_function_a>,
    pin gtx1    = d3<PfA, into_peripheral_function_a>,
    pin gcrsdv  = d4<PfA, into_peripheral_function_a>,
    pin grx0    = d5<PfA, into_peripheral_function_a>,
    pin grx1    = d6<PfA, into_peripheral_function_a>,
    pin grxer   = d7<PfA, into_peripheral_function_a>,
    pin gmdc    = d8<PfA, into_peripheral_function_a>,
    pin gmdio   = d9<PfA, into_peripheral_function_a>,
}

// The Ethernet PHY address on the SAM4E Xplained Pro
pub const PHYADDRESS: u8 = 0;

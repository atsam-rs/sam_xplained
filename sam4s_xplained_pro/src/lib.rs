#![no_std]

pub use atsam4_hal as hal;
use paste::paste;

use atsam4_hal::{define_pin_map, gpio::*, pac::MATRIX};

define_pin_map! {
    struct Pins,

    // Onboard LED
    pin led0 = c23<Output<OpenDrain>, into_open_drain_output>,

    // Onboard Button labeled SW0
    pin sw0 = c24<Input<PullUp>, into_pull_up_input>,

    // Serial Console (UART1)
    pin uart1_rx = b2<PfA, into_peripheral_function_a>,
    pin uart1_tx = b3<PfA, into_peripheral_function_a>,
}

// Note: There's two pinmaps here because both configurations
// can't be active at the same time due to the shared PC12/PC15
// pins.
define_pin_map! {
    struct ExternalMemory,

    // Static Memory Controller Pins
    pin ncs1 = c15<PfA, into_peripheral_function_a>,
    pin ncs3 = c12<PfA, into_peripheral_function_a>,

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
}

#![no_std]
#![no_main]

// Panic handler
extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use sam4s_xplained_pro::{
    hal::{
        clock::ClockController,
        delay::{Delay, DelayMs},
        gpio::Ports,
        pac::{CorePeripherals, Peripherals},
        OutputPin,
    },
    Pins,
};

#[entry]
fn main() -> ! {
    hprintln!("Blinky example started").ok();

    let core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let clocks = ClockController::new();

    // Display why a processor reset occured.
    match peripherals.RSTC.sr.read().rsttyp().bits() {
        0 => hprintln!("Reset cause: First power up reset"),
        1 => hprintln!("Reset cause: Return from backup mode"),
        2 => hprintln!("Reset cause: Watchdog timer"),
        3 => hprintln!("Reset cause: Software"),
        4 => hprintln!("Reset cause: NRST pin detected low"),
        _ => hprintln!("Reset cause: RESERVED RESET VALUE!!"),
    }
    .ok();

    let gpio_ports = Ports::new(
        peripherals.PIOA,
        clocks
            .peripheral_clocks
            .parallel_io_controller_a
            .into_enabled_clock(),
        peripherals.PIOB,
        clocks
            .peripheral_clocks
            .parallel_io_controller_b
            .into_enabled_clock(),
        peripherals.PIOC,
        clocks
            .peripheral_clocks
            .parallel_io_controller_c
            .into_enabled_clock(),
    );
    let mut pins = Pins::new(gpio_ports);
    let mut delay = Delay::new(core.SYST);

    loop {
        pins.led0.set_low().ok();
        delay.delay_ms(1000u32);
        pins.led0.set_high().ok();
        delay.delay_ms(1000u32);
    }
}

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // panic handler
use sam4s_xplained_pro::{
    hal::{
        clock::*,
        delay::{Delay, DelayMs},
        gpio::*,
        pac::{CorePeripherals, Peripherals},
        serial::Serial1,
        time::U32Ext,
        watchdog::*,
        OutputPin,
    },
    Pins,
};

#[entry]
fn main() -> ! {
    hprintln!("Serial example started").ok();

    let core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let clocks = ClockController::new(
        peripherals.PMC,
        &peripherals.SUPC,
        &peripherals.EFC0,
        &peripherals.EFC1,
        MainClock::RcOscillator12Mhz,
        SlowClock::RcOscillator32Khz,
    );

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
        (
            peripherals.PIOA,
            clocks.peripheral_clocks.pio_a.into_enabled_clock(),
        ),
        (
            peripherals.PIOB,
            clocks.peripheral_clocks.pio_b.into_enabled_clock(),
        ),
        (
            peripherals.PIOC,
            clocks.peripheral_clocks.pio_c.into_enabled_clock(),
        ),
    );
    let mut pins = Pins::new(gpio_ports);
    let mut delay = Delay::new(core.SYST);

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    let mut serial_port = Serial1::new(
        peripherals.UART1,
        clocks.peripheral_clocks.uart_1.into_enabled_clock(),
        pins.uart1_rx,
        pins.uart1_tx,
        115200_u32.bps(),
        None,
    );

    loop {
        serial_port.write_string_blocking("Hello from the serial port!\r\n");
        pins.led0.set_low().ok();
        delay.delay_ms(1000u32);
        pins.led0.set_high().ok();
        delay.delay_ms(1000u32);
    }
}

#![no_std]
#![no_main]

#[macro_use(block)]
extern crate nb;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m_systick_countdown::{MillisCountDown, PollingSysTick, SysTickCalibration};
use panic_semihosting as _; // panic handler
use sam4s_xplained_pro::{
    hal::{
        clock::*,
        gpio::*,
        pac::{CorePeripherals, Peripherals},
        serial::Serial1,
        time::rate::*,
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

    let ticker = PollingSysTick::new(
        core.SYST,
        &SysTickCalibration::from_clock_hz(get_master_clock_frequency().0),
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

    hprintln!("CPU Clock: {}", get_master_clock_frequency().0).ok();

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
    let mut pins = Pins::new(gpio_ports, &peripherals.MATRIX);

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    let mut serial_port = Serial1::new(
        peripherals.UART1,
        clocks.peripheral_clocks.uart_1.into_enabled_clock(),
        pins.uart1_rx,
        pins.uart1_tx,
        BitsPerSecond(115200_u32),
        None,
    );

    let mut counter = MillisCountDown::new(&ticker);

    loop {
        serial_port.write_string_blocking("Hello from the serial port!\r\n");
        counter.start_ms(1000);
        pins.led0.set_low().ok();
        block!(counter.wait_ms()).unwrap();

        counter.start_ms(1000);
        pins.led0.set_high().ok();
        block!(counter.wait_ms()).unwrap();
    }
}

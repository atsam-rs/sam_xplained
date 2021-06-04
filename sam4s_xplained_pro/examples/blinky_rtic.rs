#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // panic handler
use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};
use sam4s_xplained_pro::{
    hal::{clock::*, gpio::*, pac::Peripherals, watchdog::*, OutputPin},
    Pins,
};

#[app(device = sam4s_xplained_pro::hal::pac, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    //
    // Resources used by tasks/interrupts
    //
    struct Resources {
        led0: Pc23<Output<OpenDrain>>,
    }

    //
    // Initialization
    //
    #[init(schedule = [blink_led])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // Task scheduling
        cx.schedule
            .blink_led(cx.start + get_master_clock_frequency().0.cycles())
            .unwrap();

        // Resource creation
        let peripherals = Peripherals::take().unwrap();
        let clocks = ClockController::new(
            peripherals.PMC,
            &peripherals.SUPC,
            &peripherals.EFC0,
            &peripherals.EFC1,
            MainClock::RcOscillator12Mhz,
            SlowClock::RcOscillator32Khz,
        );

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

        // Turn LED0 off.
        pins.led0.set_high().ok();

        init::LateResources { led0: pins.led0 }
    }

    //
    // LED Blink Task
    //
    #[task(resources = [led0], schedule = [blink_led])]
    fn blink_led(cx: blink_led::Context) {
        static mut STATE: bool = false;

        if *STATE == false {
            cx.resources.led0.set_low().ok();
            cx.schedule
                .blink_led(Instant::now() + (get_master_clock_frequency().0 / 20).cycles())
                .unwrap();
            *STATE = true;
        } else {
            cx.resources.led0.set_high().ok();
            cx.schedule
                .blink_led(Instant::now() + (get_master_clock_frequency().0 / 2).cycles())
                .unwrap();
            *STATE = false;
        }
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn TC5(); // Timer/Count #5
    }
};

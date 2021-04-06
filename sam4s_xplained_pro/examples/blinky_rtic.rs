#![no_main]
#![no_std]

use panic_semihosting as _; // panic handler
use cortex_m_semihosting::hprintln;
use sam4s_xplained_pro::{
    hal::*,
    hal::gpio::*,
    hal::pac::Peripherals,
};
use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};

#[app(device = sam4s_xplained_pro::hal::pac, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    //
    // Resources used by tasks/interrupts
    //
    struct Resources {
        led0: gpio::Pc23<Output<OpenDrain>>,
    }

    //
    // Initialization
    //
    #[init(schedule = [blink_led])]
    fn init(mut cx: init::Context) -> init::LateResources {
        hprintln!("CPU Clock: {}", clock::get_master_clock_frequency().0).ok();

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // Task scheduling
        cx.schedule.blink_led(cx.start + clock::get_master_clock_frequency().0.cycles()).unwrap();

        // Resource creation
        let peripherals = Peripherals::take().unwrap();
        let clocks = clock::ClockController::new();
        let gpio_ports = gpio::Ports::new(
            peripherals.PIOA, 
            clocks.peripheral_clocks.parallel_io_controller_a.into_enabled_clock(),
            peripherals.PIOB, 
            clocks.peripheral_clocks.parallel_io_controller_b.into_enabled_clock(),
            peripherals.PIOC, 
            clocks.peripheral_clocks.parallel_io_controller_c.into_enabled_clock(),
        );
        let mut pins = sam4s_xplained_pro::Pins::new(gpio_ports);

        // Turn LED0 off.
        pins.led0.set_high().ok();

        init::LateResources {
            led0: pins.led0,
        }
    }

    //
    // LED Blink Task
    //
    #[task(resources = [led0], schedule = [blink_led])]
    fn blink_led(cx: blink_led::Context) {
        static mut STATE: bool = false;

        if *STATE == false {
            cx.resources.led0.set_low().ok();
            cx.schedule.blink_led(Instant::now() + (clock::get_master_clock_frequency().0 / 20).cycles()).unwrap();
            *STATE = true;
        }
        else {
            cx.resources.led0.set_high().ok();
            cx.schedule.blink_led(Instant::now() + (clock::get_master_clock_frequency().0 / 2).cycles()).unwrap();
            *STATE = false;
        }
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn TC5();    // Timer/Count #5
    }
};

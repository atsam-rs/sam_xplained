#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // panic handler
use sam4e_xplained_pro::{
    hal::{
        clock::*,
        gpio::Ports,
        pac::{CorePeripherals, Peripherals},
        static_memory_controller::{AccessMode, ChipSelectConfiguration, Smc, NCS1, NCS3},
        watchdog::*,
    },
    Pins,
};

#[entry]
fn main() -> ! {
    hprintln!("External Memory example started").ok();

    let _core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let clocks = ClockController::new(
        peripherals.PMC,
        &peripherals.SUPC,
        &peripherals.EFC,
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
        (
            peripherals.PIOD,
            clocks.peripheral_clocks.pio_d.into_enabled_clock(),
        ),
        (
            peripherals.PIOE,
            clocks.peripheral_clocks.pio_e.into_enabled_clock(),
        ),
    );
    let mut pins = Pins::new(gpio_ports, &peripherals.MATRIX);

    let smc = Smc::new(
        clocks.peripheral_clocks.smc.into_enabled_clock(),
        NCS1::D18(pins.ncs1),
        NCS3::D19(pins.ncs3),
        pins.nrd,
        pins.nwe,
        // Data Pins
        (
            pins.d0, pins.d1, pins.d2, pins.d3, pins.d4, pins.d5, pins.d6, pins.d7,
        ),
        // Address Pins
        (
            pins.a0,
            pins.a1,
            pins.a2,
            pins.a3,
            pins.a4,
            pins.a5,
            pins.a6,
            pins.a7,
            pins.a8,
            pins.a9,
            pins.a10,
            pins.a11,
            pins.a12,
            pins.a13,
            pins.a14,
            pins.a15,
            pins.a16,
            pins.a17,
            pins.a18,
            pins.a19,
            pins.a20,
            pins.a21,
            pins.a22,
            pins.a23,
        ),
    );

    let external_memory_base_address_cs1: *mut u8 = smc.base_address(1) as *mut _; // External memory using chip select 1
    let external_memory_base_address_cs3: *mut u8 = smc.base_address(3) as *mut _; // External memory using chip select 3
    const EXTERNAL_MEMORY_SIZE: usize = 512 * 1024; // Each of the above is 512 kbytes

    // Configure the Static Memory Chip Selects
    let static_memory_cs_config = ChipSelectConfiguration {
        nwe_setup_length: 1,
        ncs_write_setup_length: 1,
        nrd_setup_length: 1,
        ncs_read_setup_length: 1,

        nwe_pulse_length: 6,
        ncs_write_pulse_length: 6,
        nrd_pulse_length: 6,
        ncs_read_pulse_length: 6,

        nwe_total_cycle_length: 7,
        nrd_total_cycle_length: 7,

        access_mode: AccessMode::ReadWrite,
        wait_mode: None,

        data_float_time: 0,
        tdf_optimization: false,

        page_size: None,
    };

    smc.chip_select1
        .into_configured_state(&static_memory_cs_config);
    smc.chip_select3
        .into_configured_state(&static_memory_cs_config);

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    test_memory_region(external_memory_base_address_cs1, EXTERNAL_MEMORY_SIZE);
    test_memory_region(external_memory_base_address_cs3, EXTERNAL_MEMORY_SIZE);

    hprintln!("Testing complete without error.").ok();

    loop {}
}

fn test_memory_region(region_start_address: *mut u8, region_size_in_bytes: usize) {
    const TEST_STARTING_VALUE: u8 = 1;

    hprintln!(
        "Testing memory region located at {:?}...",
        region_start_address
    )
    .ok();

    // Write loop
    let mut write_value: u8 = TEST_STARTING_VALUE;
    for offset in 0usize..region_size_in_bytes {
        unsafe {
            core::ptr::write_volatile(region_start_address.add(offset), write_value);
            write_value = u8::wrapping_add(write_value, 1);
        }
    }

    // Check loop
    let mut value_expected = TEST_STARTING_VALUE;
    for offset in 0usize..region_size_in_bytes {
        unsafe {
            let value_read = core::ptr::read_volatile(region_start_address.add(offset));
            if value_read != value_expected {
                panic!(
                    "Memory region test failed at offset {} - read: {}, expected: {}",
                    offset, value_read, value_expected
                );
            }
            value_expected = u8::wrapping_add(value_expected, 1);
        }
    }
}

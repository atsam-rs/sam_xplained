#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // panic handler
use sam4e_xplained_pro::{
    hal::{
        clock::*,
        delay::{Delay, DelayMs},
        ethernet,
        gpio::*,
        pac::{CorePeripherals, Peripherals},
        watchdog::*,
        OutputPin,
    },
    Pins,    
};

use smoltcp::wire::{
    ArpOperation, ArpPacket, ArpRepr, EthernetAddress, EthernetFrame, EthernetProtocol,
    EthernetRepr, Ipv4Address,
};

#[entry]
fn main() -> ! {
    hprintln!("Network Pingable example started").ok();

    let core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let clocks = ClockController::new(
        peripherals.PMC,
        &peripherals.SUPC,
        &peripherals.EFC,
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
        (
            peripherals.PIOD,
            clocks.peripheral_clocks.pio_d.into_enabled_clock(),
        ),
        (
            peripherals.PIOE,
            clocks.peripheral_clocks.pio_e.into_enabled_clock(),
        ),
    );
    let mut pins = Pins::new(gpio_ports);
    let mut delay = Delay::new(core.SYST);

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    let mut eth = ethernet::Builder::new()
        .freeze(peripherals.GMAC, clocks.peripheral_clocks.gmac.into_enabled_clock());

    let mut last_status = None;

    loop {
        let status = eth.status();

        if last_status
            .map(|last_status| last_status != status)
            .unwrap_or(true)
        {
            if !status.link_detected() {
                hprintln!("Ethernet: no link detected").unwrap();
            } else {
                hprintln!(
                    "Ethernet: link detected with {} Mbps/{}",
                    status.speed(),
                    match status.is_full_duplex() {
                        true => "FD",
                        false => "HD",
                    }
                )
                .unwrap();
            }

            last_status = Some(status);
        }

        if status.link_detected() {
            const SIZE: usize = 14 + 28; // ETH + ARP

            let src_mac = ethernet::EthernetAddress::from_bytes(&[0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF]);

            let arp_buffer = [0; 28];
            let mut packet =
                ArpPacket::new_checked(arp_buffer).expect("ArpPacket: buffer size is not correct");
            let arp = ArpRepr::EthernetIpv4 {
                operation: ArpOperation::Request,
                source_hardware_addr: src_mac,
                source_protocol_addr: Ipv4Address::new(192, 168, 1, 100),
                target_hardware_addr: EthernetAddress::from_bytes(&[0x00; 6]),
                target_protocol_addr: Ipv4Address::new(192, 168, 1, 254),
            };
            arp.emit(&mut packet);

            let eth_buffer = [0; SIZE]; // ETH + ARP
            let mut frame = EthernetFrame::new_checked(eth_buffer)
                .expect("EthernetFrame: buffer size is not correct");
            let header = EthernetRepr {
                src_addr: src_mac,
                dst_addr: EthernetAddress::BROADCAST,
                ethertype: EthernetProtocol::Arp,
            };
            header.emit(&mut frame);
            frame.payload_mut().copy_from_slice(&packet.into_inner());

            let r = eth.send(SIZE, |buf| {
                buf[0..SIZE].copy_from_slice(&frame.into_inner());
            });

            match r {
                Ok(()) => {
                    hprintln!("ARP-smoltcp sent").unwrap();
                }
                Err(ethernet::TxError::WouldBlock) => hprintln!("ARP failed").unwrap(),
            }
        } else {
            hprintln!("Down").unwrap();
        }

        // cortex_m::interrupt::free(|cs| {
        //     let mut eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
        //     *eth_pending = false;

        //     if !*eth_pending {
        //         asm::wfi();
        //     }
        // });
    }
}

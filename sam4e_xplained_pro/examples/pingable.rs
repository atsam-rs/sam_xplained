#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // panic handler
use sam4e_xplained_pro::{
    hal::{
        clock::*,
        ethernet,
        gpio::*,
        pac::{CorePeripherals, Peripherals},
        watchdog::*,
    },
    Pins,    
};

use smoltcp::wire::{
    IpAddress, IpCidr,
    Ipv4Address, Icmpv4Repr, Icmpv4Packet,
};
use smoltcp::iface::{NeighborCache, EthernetInterfaceBuilder, Routes};
use smoltcp::socket::{SocketSet, IcmpSocket, IcmpSocketBuffer, IcmpPacketMetadata, IcmpEndpoint};

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

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    let mut eth = ethernet::Builder::new()
        .freeze::<16, 8>(peripherals.GMAC, clocks.peripheral_clocks.gmac.into_enabled_clock());

    // Create an IP address
    let mut ip_addrs = [IpCidr::new(IpAddress::v4(192, 168, 69, 1), 24)];

    // Create IP routes
    let mut routes_storage = [None; 1];
    let mut routes = Routes::new(&mut routes_storage[..]);
    let default_v4_gw = Ipv4Address::new(192, 168, 69, 100);
    routes.add_default_ipv4_route(default_v4_gw).unwrap();

    // Create ethernet interface
    let mut interface = EthernetInterfaceBuilder::new(eth)
        .ip_addrs(&mut ip_addrs[..])
        .routes(routes)
        .finalize();

    // Create an ICMP socket
    let mut rx_icmp_metadata_buffer:[IcmpPacketMetadata; 30] = [IcmpPacketMetadata::EMPTY; 30];
    let mut tx_icmp_metadata_buffer:[IcmpPacketMetadata; 30] = [IcmpPacketMetadata::EMPTY; 30];

    let mut rx_icmp_payload_buffer:[u8; 30] = [0; 30];
    let mut tx_icmp_payload_buffer:[u8; 30] = [0; 30];

    let icmp_rx_buffer = IcmpSocketBuffer::new(rx_icmp_metadata_buffer, rx_icmp_payload_buffer);
    let icmp_tx_buffer = IcmpSocketBuffer::new(tx_icmp_metadata_buffer, tx_icmp_payload_buffer);
    let icmp_socket = IcmpSocket::new(icmp_rx_buffer, icmp_tx_buffer);

    let mut link_detected = false;
    loop {
        let status = interface.device().status();
        if status.link_detected() != link_detected {
            if status.link_detected() {
                hprintln!("Ethernet link is no UP with {} Mbps.", status.speed()).unwrap();
            } else {
                hprintln!("Ethernet link is now DOWN.").unwrap();
            }

            link_detected = status.link_detected();
        }

        if status.link_detected() {
//            let timestamp = Instant::now();
            match interface.poll(&mut sockets, timestamp) {
                Ok(_) => {},
                Err(e) => {
                    hprintln!("poll error: {}", e).unwrap();
                }
            }
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

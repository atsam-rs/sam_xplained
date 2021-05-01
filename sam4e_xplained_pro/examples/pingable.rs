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
use smoltcp::socket::{SocketSetItem, SocketSet, IcmpSocket, IcmpSocketBuffer, IcmpPacketMetadata, IcmpEndpoint};
use smoltcp::time::*;

use byteorder::{ByteOrder, NetworkEndian};

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
    
    // let gpio_ports = Ports::new(
    //     (
    //         peripherals.PIOA,
    //         clocks.peripheral_clocks.pio_a.into_enabled_clock(),
    //     ),
    //     (
    //         peripherals.PIOB,
    //         clocks.peripheral_clocks.pio_b.into_enabled_clock(),
    //     ),
    //     (
    //         peripherals.PIOC,
    //         clocks.peripheral_clocks.pio_c.into_enabled_clock(),
    //     ),
    //     (
    //         peripherals.PIOD,
    //         clocks.peripheral_clocks.pio_d.into_enabled_clock(),
    //     ),
    //     (
    //         peripherals.PIOE,
    //         clocks.peripheral_clocks.pio_e.into_enabled_clock(),
    //     ),
    // );
    // let mut pins = Pins::new(gpio_ports);

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    let mut eth = ethernet::Builder::new()
        .freeze::<16, 8>(peripherals.GMAC, clocks.peripheral_clocks.gmac.into_enabled_clock());

    // Create an IP address
    let mut ip_addrs = [IpCidr::new(IpAddress::v4(192, 168, 1, 242), 24)];

    // Create IP routes
    let mut routes_storage = [None; 1];
    let mut routes = Routes::new(&mut routes_storage[..]);
    let default_v4_gw = Ipv4Address::new(192, 168, 1, 1);
    routes.add_default_ipv4_route(default_v4_gw).unwrap();

    // Create ethernet interface
    let mut interface = EthernetInterfaceBuilder::new(eth)
        .ip_addrs(&mut ip_addrs[..])
        .routes(routes)
        .finalize();

    // Create an ICMP socket
    // let mut rx_icmp_metadata_buffer:[IcmpPacketMetadata; 30] = [IcmpPacketMetadata::EMPTY; 30];
    // let mut tx_icmp_metadata_buffer:[IcmpPacketMetadata; 30] = [IcmpPacketMetadata::EMPTY; 30];

    // let mut rx_icmp_payload_buffer:[u8; 30] = [0; 30];
    // let mut tx_icmp_payload_buffer:[u8; 30] = [0; 30];

    // let icmp_rx_buffer = IcmpSocketBuffer::new(&mut rx_icmp_metadata_buffer[..], &mut rx_icmp_payload_buffer[..]);
    // let icmp_tx_buffer = IcmpSocketBuffer::new(&mut tx_icmp_metadata_buffer[..], &mut tx_icmp_payload_buffer[..]);
    // let icmp_socket = IcmpSocket::new(icmp_rx_buffer, icmp_tx_buffer);

    let mut socket_array:[Option<SocketSetItem>; 5] = [None, None, None, None, None];
    let mut sockets = SocketSet::new(&mut socket_array[..]);
    // let icmp_handle = sockets.add(icmp_socket);
    // let ident = 0x22b;
    // let mut send_at = Instant::from_millis(0);

    let mut link_detected = false;
    // let mut echo_payload = [0xffu8; 40];

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
            let timestamp = Instant::from_millis(0);
            match interface.poll(&mut sockets, timestamp) {
                Ok(_) => {},
                Err(e) => {
                    hprintln!("poll error: {}", e).unwrap();
                }
            }

            // let timestamp = Instant::from_millis(0u32);
            // let mut socket = sockets.get::<IcmpSocket>(icmp_handle);
            // if !socket.is_open() {
            //     socket.bind(IcmpEndpoint::Ident(ident)).unwrap();
            //     send_at = timestamp;
            // }

            // if socket.can_send()  {
            //     NetworkEndian::write_i64(&mut echo_payload, timestamp.total_millis());

            //     match remote_addr {
            //         IpAddress::Ipv4(_) => {
            //             let (icmp_repr, mut icmp_packet) = send_icmp_ping!(
            //                     Icmpv4Repr, Icmpv4Packet, ident, seq_no,
            //                     echo_payload, socket, remote_addr);
            //             icmp_repr.emit(&mut icmp_packet, &device_caps.checksum);
            //         },
            //         IpAddress::Ipv6(_) => {
            //             let (icmp_repr, mut icmp_packet) = send_icmp_ping!(
            //                     Icmpv6Repr, Icmpv6Packet, ident, seq_no,
            //                     echo_payload, socket, remote_addr);
            //             icmp_repr.emit(&src_ipv6, &remote_addr,
            //                             &mut icmp_packet, &device_caps.checksum);
            //         },
            //         _ => unimplemented!()
            //     }

            //     waiting_queue.insert(seq_no, timestamp);
            //     seq_no += 1;
            //     send_at += interval;
            // }

            // if socket.can_recv() {
            //     let (payload, _) = socket.recv().unwrap();

            //     match remote_addr {
            //         IpAddress::Ipv4(_) => {
            //             let icmp_packet = Icmpv4Packet::new_checked(&payload).unwrap();
            //             let icmp_repr =
            //                 Icmpv4Repr::parse(&icmp_packet, &device_caps.checksum).unwrap();
            //             get_icmp_pong!(Icmpv4Repr, icmp_repr, payload,
            //                     waiting_queue, remote_addr, timestamp, received);
            //         },
            //         _ => unimplemented!()
            //     }
            // }

            // waiting_queue.retain(|seq, from| {
            //     if timestamp - *from < timeout {
            //         true
            //     } else {
            //         hprintln!("From {} icmp_seq={} timeout", remote_addr, seq);
            //         false
            //     }
            // });
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

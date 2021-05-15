#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // panic handler
use sam4e_xplained_pro::{
    hal::{
        clock::*,
        ethernet,
        pac::{CorePeripherals, Peripherals},
        watchdog::*,
    },
};

use smoltcp::wire::{Ipv4Address, IpCidr, Ipv4Cidr};
use smoltcp::iface::{NeighborCache, EthernetInterfaceBuilder, Routes};
use smoltcp::socket::{SocketSet, SocketSetItem, RawSocketBuffer, RawPacketMetadata};
use smoltcp::time::Instant;
use smoltcp::dhcp::Dhcpv4Client;
use smoltcp::wire::EthernetAddress;

static mut RXDESCRIPTORBLOCK: ethernet::RxDescriptorBlock<8> = ethernet::RxDescriptorBlock::<8>::const_default();
static mut TXDESCRIPTORBLOCK: ethernet::TxDescriptorBlock<4> = ethernet::TxDescriptorBlock::<4>::const_default();

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

    // Disable the watchdog timer.
    Watchdog::new(peripherals.WDT).disable();

    //
    // Ethernet controller setup
    //
    let eth = {
        unsafe {
            RXDESCRIPTORBLOCK.initialize(&peripherals.GMAC);
            TXDESCRIPTORBLOCK.initialize(&peripherals.GMAC);

            ethernet::Builder::new()
                .freeze(
                    peripherals.GMAC, 
                    clocks.peripheral_clocks.gmac.into_enabled_clock(), 
                    &mut RXDESCRIPTORBLOCK,
                    &mut TXDESCRIPTORBLOCK)
        }
    };

    let mut ip_addrs = [IpCidr::new(Ipv4Address::UNSPECIFIED.into(), 0)];
    let mut routes_storage = [None; 1];
    let routes = Routes::new(&mut routes_storage[..]);
    
    // Are these strictly necessary?
    let ethernet_addr = EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]);
    let mut neighbor_cache_entries = [None; 8];
    let mut neighbor_cache = NeighborCache::new(&mut neighbor_cache_entries[..]);

    // Create ethernet interface
    let mut interface = EthernetInterfaceBuilder::new(eth)
        .ethernet_addr(ethernet_addr)
        .neighbor_cache(neighbor_cache)
        .ip_addrs(&mut ip_addrs[..])
        .routes(routes)
        .finalize();

    let mut socket_storage:[Option<SocketSetItem>; 5] = [None, None, None, None, None,];
    let mut sockets = SocketSet::new(&mut socket_storage[..]);

    let mut dhcp_rx_metadata_buffer:[RawPacketMetadata; 1] = [RawPacketMetadata::EMPTY; 1];
    let mut dhcp_tx_metadata_buffer:[RawPacketMetadata; 1] = [RawPacketMetadata::EMPTY; 1];

    let mut dhcp_rx_payload_buffer: [u8; 900] = [0; 900];
    let mut dhcp_tx_payload_buffer: [u8; 600] = [0; 600];

    let dhcp_rx_buffer = RawSocketBuffer::new(
        &mut dhcp_rx_metadata_buffer[..],
        &mut dhcp_rx_payload_buffer[..]
    );
    let dhcp_tx_buffer = RawSocketBuffer::new(
        &mut dhcp_tx_metadata_buffer[..],
        &mut dhcp_tx_payload_buffer[..]
    );

    let mut dhcp = Dhcpv4Client::new(&mut sockets, dhcp_rx_buffer, dhcp_tx_buffer, Instant::from_millis(0));
    let mut prev_cidr = Ipv4Cidr::new(Ipv4Address::UNSPECIFIED, 0);
    
    let mut link_detected: Option<bool> = None;

    loop {
        let status = interface.device().status();
        if link_detected.is_none() || status.link_detected() != link_detected.unwrap() {
            if status.link_detected() {
                hprintln!("Ethernet link is now UP with {} Mbps.", status.speed()).unwrap();
            } else {
                hprintln!("Ethernet link is now DOWN.").unwrap();
            }

            link_detected = Some(status.link_detected());
        }

        if status.link_detected() {
            let timestamp = Instant::from_millis(0);
            interface.poll(&mut sockets, timestamp)
                .map(|_| ())
                .unwrap_or_else(|e| hprintln!("Poll: {:?}", e).unwrap());

            let config = dhcp.poll(&mut interface, &mut sockets, timestamp)
            .unwrap_or_else(|e| {
                hprintln!("DHCP: {:?}", e).unwrap();
                None
            });
       }
    }
}

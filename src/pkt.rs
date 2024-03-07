
use packet::ip::v4::Packet as V4Packet;
use packet::ip::v6::Packet as V6Packet;
use packet::ip::Packet as IPPacket;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use packet::Packet;

impl DataPacket for V4Packet<Vec<u8>> {
    fn src(&self) -> IpAddr {
        IpAddr::V4(self.source())
    }
    fn dest(&self) -> IpAddr {
        IpAddr::V4(self.destination())
    }
    fn data(&self) -> &[u8] {
        self.payload()
    }
}

impl DataPacket for V6Packet<Vec<u8>> {
    fn src(&self) -> IpAddr {
        IpAddr::V6(self.source())
    }
    fn dest(&self) -> IpAddr {
        IpAddr::V6(self.destination())
    }
    fn data(&self) -> &[u8] {
        self.payload()
    }
}

impl DataPacket for IPPacket<Vec<u8>> {
    fn src(&self) -> IpAddr {
        match self {
            IPPacket::V4(addr) => addr.src(),
            IPPacket::V6(addr) => addr.src(),
        }
    }
    fn dest(&self) -> IpAddr {
        match self {
            IPPacket::V4(addr) => addr.dest(),
            IPPacket::V6(addr) => addr.dest(),
        }
    }
    fn data(&self) -> &[u8] {
        match self {
            IPPacket::V4(pkt) => {
                pkt.data()
            },
            IPPacket::V6(pkt) => {
                pkt.data()
            },
        }
    }
}

pub trait DataPacket: std::marker::Send {
    fn src(&self) -> IpAddr;
    fn dest(&self) -> IpAddr;
    fn data(&self) -> &[u8];
    //fn protocol(&self) -> u8;
    //fn id(&self) -> u16;
    //fn ttl(&self) -> u8;
}
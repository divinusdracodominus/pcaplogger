#[macro_use]
extern crate serde_derive;
use async_trait::async_trait;
use err_derive::Error;
use packet::ip::Packet as IPPacket;
use packet::ether::Packet as Frame;
use packet::ether::Protocol;
use pcap::Device;
use std::net::IpAddr;
use tokio::task::JoinHandle;
use hwaddr::HwAddr;
use packet::Packet;

pub mod config;
pub mod pkt;
use pkt::DataPacket;

#[cfg(feature = "postgresql")]
pub mod postgres;

#[derive(Debug, Error)]
pub enum Empty {}

#[derive(Debug, Error)]
pub enum CapError<E: std::error::Error + std::fmt::Debug = Empty> {
    #[error(display = "device {} not found", _0)]
    DevNotFound(String),
    #[error(display = "{}", _0)]
    PCapError(#[source] pcap::Error),
    #[error(display = "unrecognized version: {}", _0)]
    VersionError(u8),
    #[error(display = "{}", _0)]
    PacketErr(#[source] packet::Error),
    #[error(display = "{}", _0)]
    Custom(E),
}

pub trait DataFrame {
    fn hw_src(&self) -> HwAddr;
    fn hw_dest(&self) -> HwAddr;
    fn data(&self) -> &[u8];
}

impl DataFrame for Frame<&[u8]> {
    fn hw_src(&self) -> HwAddr {
        self.source()
    }
    fn hw_dest(&self) -> HwAddr {
        self.destination()
    }
    fn data(&self) -> &[u8] {
        self.payload()
    }
}

#[async_trait]
pub trait Logger<E: std::error::Error>: std::marker::Send + 'static {
    async fn log<P: DataPacket>(&mut self, interface: &str, packet: P) -> Result<usize, CapError<E>>;
}

pub struct PacketCapture {}

impl PacketCapture {
    pub async fn begin<E: std::error::Error + std::fmt::Debug + std::marker::Send + 'static, L: Logger<E>>(interface: String, mut logger: L) -> Result<usize, CapError<E>> {
        tokio::spawn(async move {
            let mut device = if let Some(device) = Device::list()?
                .into_iter()
                .filter(|device| device.name == interface)
                .collect::<Vec<Device>>()
                .pop()
            {
                device.open()?
            } else {
                return Err(CapError::<E>::DevNotFound(interface));
            };
            let mut count = 0;
            
            while let Ok(packet) = device.next_packet() {
                count = count + 1;
                let frame = Frame::new(packet.data).unwrap();
                if frame.data().len() > 0 && (
                        frame.protocol() == Protocol::Ipv4 || frame.protocol() == Protocol::Ipv6) {
                    let pkt = IPPacket::new(frame.data()).unwrap().to_owned();
                    logger.log(&interface, pkt).await?;
                    if count % 1000 == 0 {
                        println!("logged: {}", count);
                    }
                }
            }
            Ok(count)
        }).await.unwrap()
    }
}

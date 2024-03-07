use core::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logger {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Name(String),
    Identifier(u8),
    Protocol(Box<NetworkProtocol>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Filter {
    Source(IpAddr),
    Dest(IpAddr),
    Protocol(u8),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub interface: String,
    pub logger: Logger,
    pub filters: Option<Vec<Filter>>,
}
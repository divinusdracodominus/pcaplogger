use crate::{CapError, Logger, pkt::DataPacket};
use tokio_postgres::Client;
use async_trait::async_trait;

impl From<tokio_postgres::Error> for CapError<tokio_postgres::Error> {
    fn from(error: tokio_postgres::Error) -> CapError<tokio_postgres::Error> {
        CapError::Custom(error)
    }
}

#[async_trait]
impl Logger<tokio_postgres::Error> for Client {
    async fn log<P: DataPacket>(&mut self, interface: &str, pkt: P) -> Result<usize, CapError<tokio_postgres::Error>> {
        let stmt = self.prepare("insert into packet_capture (interface, id, source, destination, protocol, data) values ($1, $2, $3, $4, $5, $6)").await?;
        let count = self.execute(&stmt, &[&interface, &0, &pkt.src(), &pkt.dest(), &0, &pkt.data()]).await?;
        Ok(count as usize)
    }
}
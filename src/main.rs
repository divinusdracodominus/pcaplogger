#[macro_use]
extern crate tokio;
use structopt::StructOpt;
use std::io::Read;
use capture::{PacketCapture, config::Config};
use tokio_postgres::NoTls;
use std::path::PathBuf;

#[derive(Debug, Clone, StructOpt)]
pub struct Args {
    #[structopt(short, long)]
    configuration: PathBuf,
}


#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let config = args.configuration;
    let mut contents = String::new();
    let mut file = std::fs::File::open(&config).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let conf: Config = serde_json::from_str(&contents).unwrap();
    let url = conf.logger.url;

    let (client, connection) =
        tokio_postgres::connect(&url, NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("about to begin");
    PacketCapture::begin(conf.interface, client).await.unwrap();
}

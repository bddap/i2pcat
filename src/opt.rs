use crate::run::{listen, send};
use i2p::net::I2pAddr;
use std::string::ToString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "i2pcat", about = "Cat over i2p.")]
pub enum Opt {
    #[structopt(name = "listen")]
    Listen {},
    #[structopt(name = "send")]
    Send {
        #[structopt(parse(from_str = "I2pAddr::new"))]
        i2p_addr: I2pAddr,
        port: u16,
    },
}

impl Opt {
    pub fn run(self) {
        match self {
            Opt::Listen {} => listen(),
            Opt::Send { i2p_addr, port } => send(i2p_addr, port),
        }
    }
}

use i2p::net::*;
use std::io;

pub fn listen() {
    let server = I2pListener::bind().unwrap();
    let local_addr = server.local_addr().unwrap();
    let our_dest = local_addr.dest().string();
    let our_port = local_addr.port();
    debug_assert_eq!(
        local_addr,
        I2pSocketAddr::new(I2pAddr::new(&our_dest), our_port)
    );
    eprintln!("{:?} {}", our_dest, our_port);
    let (mut stream, _i2psocket_addr) = server.accept().unwrap();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    io::copy(&mut stream, &mut stdout).unwrap();
}

pub fn send(i2p_addr: I2pAddr, port: u16) {
    let mut sock = I2pStream::connect(I2pSocketAddr::new(i2p_addr, port)).unwrap();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    io::copy(&mut stdin, &mut sock).unwrap();
}

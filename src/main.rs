use std::net::UdpSocket;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

async fn send() -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:0");
    let socket = UdpSocket::bind(bind_addr)?;
    let tmp = RandomState::new().build_hasher().finish().to_ne_bytes();
    let addr = format!("{}.{}.{}.{}:{}", tmp[0], tmp[1], tmp[2], tmp[3], tmp[4]);
    socket.send_to(b"hello", addr)?;
    Ok(())
}

fn main() {
    loop {
        smolscale::spawn(async move {
            send().await
        }).detach();
    }
}

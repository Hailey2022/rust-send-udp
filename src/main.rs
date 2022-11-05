use std::net::UdpSocket;

fn send(ip: &str, port: u16) -> anyhow::Result<()> {
    let addr = format!("{}:{}", ip, port);
    let socket = UdpSocket::bind("0.0.0.0:8888")?;
    socket.send_to(b"hello", addr)?;
    Ok(())
}

fn main() {
    let mut port = 1u16;
    loop {
        if let Err(e) = send("114.114.114.114", port) {
            eprintln!("{e}");
        }
        if port < 65535 {
            port += 1;
        }else{
            port = 1;
        }
    }
}
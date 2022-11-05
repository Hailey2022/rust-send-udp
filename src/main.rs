use smol::net::UdpSocket;

async fn send(ip: &str, port: u16) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", port);
    let addr = format!("{}:{}", ip, port);
    let socket = UdpSocket::bind(bind_addr).await?;
    socket.send_to(b"hello", addr).await?;
    Ok(())
}

fn main() {
    let mut port = 1u16;
    loop {
        smolscale::spawn(async move {
            send("114.114.114.114", port).await
        }).detach();
        if port < 65535 {
            port += 1;
        } else {
            port = 1;
        }
    }
}
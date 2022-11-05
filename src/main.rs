// use std::io::{BufRead, Write};
use smol::net::UdpSocket;

async fn send(ip: &str, local_port: u16) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", local_port);
    let socket = UdpSocket::bind(bind_addr).await?;
    for port in 1..65535u16 {
        let addr = format!("{}:{}", ip, port);
        socket.send_to(b"hello", addr).await?;
    }
    Ok(())
}

// fn prompt(message: &str) -> anyhow::Result<String> {
//     let stdout = std::io::stdout();
//     let mut stdout = stdout.lock();
//     stdout.write_all(message.as_bytes())?;
//     stdout.flush()?;
//     let stdin = std::io::stdin();
//     let mut stdin = stdin.lock();
//     let mut line = String::new();
//     stdin.read_line(&mut line)?;
//     Ok(line)
// }

fn main() {
//     let ip = match prompt("ip:") {
//         Ok(ip) => {
//             println!("using {ip}");
//             ip
//         },
//         Err(e) => {
//             println!("using 1.1.1.1 since {e}");
//             String::from("1.1.1.1")
//         }
//     };
    let mut port = 1u16;
    loop {
        smolscale::spawn(async move {
            send("114.114.114.114", port).await
        }).detach();
        if port < 65535 {
            port += 1;
        } else {
            println!("again!");
            port = 1;
        }
    }
}

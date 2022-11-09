// use std::io::{BufRead, Write};
use std::net::UdpSocket;

fn send(ip: &str) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:0");
    let socket = UdpSocket::bind(bind_addr)?;
    for port in 1..65535u16 {
        let addr = format!("{}:{}", ip, port);
        socket.send_to(b"hello", addr)?;
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
//     let mut port = 1u16;
    loop {
            if let Err(_e) = send("114.114.114.114") {
                continue
            }
//         if port < 65535 {
//             port += 1;
//         } else {
//             println!("again!");
//             port = 1;
//         }
    }
}

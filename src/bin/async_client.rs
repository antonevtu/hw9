use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, mpsc::Receiver, mpsc::Sender};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    let tx_cln = tx.clone();

    tokio::spawn(async move {
        run_udp_client(tx_cln).await;
    });

    tokio::spawn(async move {
        run_tcp_client(tx).await;
    });

    print_result(rx).await;
    println!("finished!")
}

async fn run_udp_client(tx: Sender<String>) {
    let addr = "127.0.0.1:5507";
    let socket = UdpSocket::bind(addr).await.unwrap();

    tokio::spawn(async move {
        let mut buf = [0u8; 4];
        for _ in 0..10 {
            let _ = socket.recv_from(&mut buf).await.unwrap();
            let t = f32::from_be_bytes(buf);
            let msg = format!("Message from thermometer: {} C", t);
            tx.send(msg).await.unwrap();
        }
    });
}

async fn run_tcp_client(tx: Sender<String>) {
    let mut stream = TcpStream::connect("127.0.0.1:5555").await.unwrap();

    let mut buf = vec![0u8; 128];

    let commands: [i32; 8] = [2, 1, 2, 3, 0, 3, 1, 3];

    for command in commands {
        match command {
            0 => println!("Send command to smart socket: turn off"),
            1 => println!("Send command to smart socket: turn on"),
            2 => println!("Send command to smart socket: get state"),
            3 => println!("Send command to smart socket: get power"),
            _ => println!("unknown command"),
        }

        stream.write_all(&command.to_be_bytes()).await.unwrap();
        let n = stream.read(&mut buf).await.unwrap();

        let mut reply = String::from("Message from smart socket: ");
        reply.push_str(str::from_utf8(&buf[..n]).unwrap());

        tx.send(reply.to_string()).await.unwrap();
    }
}

async fn print_result(mut rx: Receiver<String>) {
    while let Some(message) = rx.recv().await {
        println!("{}", message);
    }
}

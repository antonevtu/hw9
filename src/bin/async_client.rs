use tokio::sync::mpsc;
use tokio::net::UdpSocket;


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5507";
    let socket = UdpSocket::bind(addr).await.unwrap();

    let (mut tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        let mut buf = [0u8; 4];
        for _ in 0..10 {
            let _ = socket.recv_from(&mut buf).await.unwrap();
            let t = f32::from_be_bytes(buf);
            tx.send(t).await.unwrap();
        }
    });

    // tokio::spawn(async move {
    //     for t in rx {
    //         println!("Temperature: {}", t)
    //     }
    // }

    println!("finished!")
}

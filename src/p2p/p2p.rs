use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {

    // 클라이언트로부터 메시지를 받음.
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    // 받은 메시지 출력
    println!("Received: {}", String::from_utf8_lossy(&buffer));
    // 
    stream.write_all(b"Message received")?;

    Ok(())

}


pub fn p2p_server() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080...");

    // 클라이언트 연결 수락하고, 각 연결에 대해 새로운 스레드를 생성하여 처리
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

}

pub fn p2p_client() -> std::io::Result<()> {
    // 연결할 TCP server IP & Port
    let server_address = "127.0.0.1:8081";

    // Connect to TCP server
    let mut stream = TcpStream::connect(server_address)?;

    // send Message to Server

    let message = "I am node 1";
    let mut counter = 0;
    loop {
        stream.write_all(message.as_bytes())?;
        println!("Message sent to server : {}, {}", message, counter);
        counter += 1;
        thread::sleep(Duration::from_secs(1));

    }


    //Ok(())

}

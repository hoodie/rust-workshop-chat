/*

* readline loop
* readline send loop
* parse args for address and port

* parse args for send or recv option
* add recv and print loop

* make send and recv both available with thread for each

*/

use std::io::{BufRead, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use std::{env, io, net, thread};

fn main() {
    color_backtrace::install();
    let ip: IpAddr = env::args()
        .nth(1)
        .as_deref()
        .unwrap_or("127.0.0.1")
        .parse()
        .unwrap();
    let port = env::args()
        .nth(2)
        .as_deref()
        .unwrap_or("1234")
        .parse()
        .unwrap();

    let listen_addr = (Ipv4Addr::UNSPECIFIED, port);
    let connect_addr = (ip, port);

    let mut server_thread: Option<thread::JoinHandle<_>> = None;

    if let Ok(listener) = net::TcpListener::bind(listen_addr) {
        println!("--- bound to {}", listener.local_addr().unwrap());
        server_thread = Some(thread::spawn(move || loop_server(listener)));
    }

    println!("--- connect to {}:{}", ip, port);

    let mut stream = net::TcpStream::connect(connect_addr).unwrap();

    println!("--- connected");

    let client_thread = thread::spawn(move || loop_client(&mut stream));

    if let Some(handle) = server_thread {
        handle.join().unwrap();
    }
    client_thread.join().unwrap();
}

fn loop_client(stream: &mut net::TcpStream) {
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(&input.as_bytes()).unwrap();
        println!("--- send {}", input.trim())
    }
}

enum ServerMsg {
    MsgFromClient(String, net::SocketAddr),
    AcceptStream(net::TcpStream, net::SocketAddr),
}

fn loop_server(listener: net::TcpListener) {
    let mut connection_threads: Vec<thread::JoinHandle<()>> = Vec::new();

    let (msg_producer, msg_consumer) = channel();

    let msg_from_client_producer = msg_producer.clone();
    let server_thread = thread::spawn(move || {
        for msg in msg_consumer.into_iter() {
            match msg {
                ServerMsg::MsgFromClient(msg, addr) => {
                    println!("--- recv from {}", addr);
                    print!("{}", msg);
                }
                ServerMsg::AcceptStream(stream, addr) => {
                    println!("--- accept {}", addr);

                    let msg_from_client_producer = msg_from_client_producer.clone();
                    connection_threads.push(thread::spawn(move || {
                        loop_connection(&stream, &addr, &msg_from_client_producer)
                    }));
                }
            }
        }

        for handle in connection_threads.into_iter() {
            handle.join().unwrap();
        }
    });

    let accept_stream_producer = msg_producer.clone();
    let accept_thread = thread::spawn(move || {
        while let Ok((stream, address)) = listener.accept() {
            accept_stream_producer
                .send(ServerMsg::AcceptStream(stream, address))
                .unwrap();
        }
    });

    accept_thread.join().unwrap();
    server_thread.join().unwrap();
}

fn loop_connection(stream: &net::TcpStream, address: &net::SocketAddr, sender: &Sender<ServerMsg>) {
    let mut reader = io::BufReader::new(stream);

    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);

        if result.unwrap() == 0 {
            println!("--- left from {}", address);
            break;
        } else {
            sender
                .send(ServerMsg::MsgFromClient(line.clone(), address.clone()))
                .unwrap();
        }
    }
}

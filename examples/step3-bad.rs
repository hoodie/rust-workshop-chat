#![allow(unused_imports)]
use std::io::{BufRead, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::{env, io, thread};

/// only one minor change over step1
fn send_loop(addr: &str) {
    println!("sending to {:#?}", addr);

    let mut stream = TcpStream::connect(addr).unwrap();
    // input buffer for stdin, preallocated with
    let mut input = String::with_capacity(100);
    loop {
        eprint!("> ");
        io::stdin().read_line(&mut input).unwrap();
        if input == "exit\n" { break }
        stream.write_all(&input.as_bytes()).unwrap();
        input.clear();
    }
}

/// Nothing new here
fn recv_loop(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();

    println!("listening on {:?}", listener.local_addr());

    while let Ok((tcp_stream, addr)) = listener.accept() {
        println!("Connection received! {:?} is sending data.", addr);
        let mut stream_reader = io::BufReader::new(tcp_stream);

        loop {
            let mut line = String::new();
            match stream_reader.read_line(&mut line) {
                Ok(0) => {
                    println!("connection closed");
                    break;
                }
                _ => print!("# {}", line),
            }
        }
    }
}

/// THIS IS NEW
fn both_loops(recv_addr: &str, send_addr: &str) {
    {
        // make a copy of the recv_addr let it be moved into the closure scope of thread::spawn
        let recv_addr = recv_addr.to_owned();
        thread::spawn(move || recv_loop(&recv_addr));
    }
    println!("waiting 5s to connect to remote");

    // give ourselves some time to open the other side
    thread::sleep(Duration::from_secs(5));
    send_loop(&send_addr);
}

fn main() {
    color_backtrace::install();

    // we can match for multiple things at once
    match (
        env::args().nth(1).as_deref(),
        env::args().nth(2),
        env::args().nth(3),
    ) {
        (Some("send"), Some(addr), _) => send_loop(&addr),
        (Some("recv"), Some(addr), _) => recv_loop(&addr),
        (Some("both"), Some(send_addr), Some(recv_addr)) => both_loops(&send_addr, &recv_addr),
        _ => {
            println!("please supply on of the following options");
            println!("$ send send_addr");
            println!("$ recv recv_addr");
            println!("$ both send_addr recv_addr");
        }
    }
}

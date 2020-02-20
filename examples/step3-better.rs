#![allow(unused_imports)]
use std::io::{BufRead, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::{env, io, thread};

/// only one minor change over step1
fn send_loop(mut stream: TcpStream) {
    println!("sending to {:#?}", stream.local_addr());

    // input buffer for stdin, preallocated with
    let mut input = String::with_capacity(100);
    loop {
        eprint!("> ");
        io::stdin().read_line(&mut input).unwrap();
        if input == "exit\n" { break }
        println!("input {:?}",input);
        stream.write_all(&input.as_bytes()).unwrap();
        input.clear();
    }
}

/// Nothing new here
fn recv_loop(addr: &str) {

    if let Ok(read_stream) = TcpStream::connect(addr) {
        println!("Connection opened! {:?}.", addr);

        let write_stream = read_stream.try_clone().unwrap();
        thread::spawn(|| send_loop(write_stream));

        let mut stream_reader = io::BufReader::new(read_stream);
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

fn main() {
    color_backtrace::install();

    if let Some(addr) = env::args().nth(1) {
        recv_loop(&addr);
    }
}

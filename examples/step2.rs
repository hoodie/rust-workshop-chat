//! # Content
//!
//! 1. listen for tcp connections
//! 2. BufReader
//! 3. 
//! 

#![allow(unused_imports)]
use std::io::{BufRead, Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpListener};
use std::sync::mpsc::{channel, Sender};
use std::{env, io, thread};

fn main() {
    // for prettier backtraces
    color_backtrace::install();

    // the the second element from the args iterator
    if let Some(addr) = env::args().nth(1) {
        // listen for incoming tcp connections
        let listener = TcpListener::bind(addr).unwrap();
        println!("listening on {:?}", listener.local_addr());

        // for each accepted connection
        while let Ok((tcp_stream, remote)) = listener.accept() {
            thread::spawn(move || {
                println!("connection received from {:?}", remote);

                // buffers tcp incoming tcp content so we can read it line by line
                let mut stream_reader = io::BufReader::new(tcp_stream);

                // recv buffer
                let mut line = String::new();
                loop {
                    match stream_reader.read_line(&mut line) {
                        Ok(0) => {
                            // received a 0byte package
                            println!("connection closed");
                            break;
                        }
                        _ => {
                            println!("{} {}", remote, line.trim());
                            line.clear();
                        }
                    }
                }
            });
        }
    } else {
        println!("please add command line arguments")
    }
}

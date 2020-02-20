//! # Content
//!
//! 1. how to parse args
//! 2. read from stdin
//! 3. open a TcpSocket to send strings


use std::io::Write;
use std::net::TcpStream;
use std::{env, io};

fn main() {
    // for prettier backtraces
    color_backtrace::install();

    // the the second element from the args iterator
    if let Some(addr) = env::args().nth(1) {
        println!("only sending to {:#?}", addr);

        // open writable stream tcp stream
        let mut stream = TcpStream::connect(addr).unwrap();

        // input buffer for stdin, preallocated with
        let mut input = String::new();
        loop {
            // pretend we're a shell
            eprint!("> ");

            // read one line from std into input
            io::stdin().read_line(&mut input).unwrap();
            if input == "exit\n" { break }

            // write it on the stream
            stream.write_all(&input.as_bytes()).unwrap();
            input.clear();
        }
    } else {
        println!("please pass a sending address")
    }
}

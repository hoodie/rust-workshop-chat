//! # Content
//! 1. Channel

#![allow(unused_imports)]
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, io, thread};

use env_logger::Env;
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
enum BrokerCmd {
    NewPeer {
        // name: String,
        sender: Sender<PeerMessage>,
    },
    Message {
        content: String,
    },
}
#[derive(Debug)]
enum PeerMessage {
    Message { content: String },
    Close,
}

trait MsgHandler {}

struct Server<T: MsgHandler> {
    addr: String,
    handler: T,
}

impl<T: MsgHandler> Server<T> {
    fn start(self) {
        Self::accept_loop(&self.addr);
    }

    fn accept_loop(addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        info!("listening on {:?}", listener.local_addr());

        // every time a new connection is opened
        let (broker_tx, broker_rx) = channel::<BrokerCmd>();

        thread::spawn(move || {
            Self::broker_loop(broker_rx);
        });

        // new connection comes i
        while let Ok((tcp_stream, addr)) = listener.accept() {
            info!("connection from {:?}", addr);

            // we create a new thread and hand it a way to contact us
            let tx = broker_tx.clone();
            let (peer_tx, peer_rx) = channel::<PeerMessage>();

            thread::spawn(|| Self::connection_recv_loop(tx, peer_rx, tcp_stream));

            broker_tx
                .send(BrokerCmd::NewPeer { sender: peer_tx })
                .unwrap();
        }
    }

    fn connection_recv_loop(
        to_broker: Sender<BrokerCmd>,
        peer_rx: Receiver<PeerMessage>,
        read_stream: TcpStream,
    ) {
        info!(
            "receive loop started {:?}",
            read_stream.peer_addr().unwrap()
        );

        let mut line = String::new();
        let mut stream_reader = io::BufReader::new(&read_stream);
        let remote_addr = read_stream.peer_addr().unwrap();

        let mut write_stream = read_stream.try_clone().unwrap();
        thread::spawn(move || Self::connection_send_loop(peer_rx, &mut write_stream));

        loop {
            match stream_reader.read_line(&mut line) {
                Ok(0) => {
                    info!("connection closed {:?}", remote_addr);
                    // to_broker.send(BrokerCmd::Close).unwrap();
                    break;
                }
                Ok(_) => {
                    debug!("received message from {:?} {:?}", remote_addr, line);
                    let msg = BrokerCmd::Message {
                        content: line.clone(),
                    };
                    line.clear();
                    to_broker.send(msg).unwrap();
                }
                _ => {
                    warn!("unexpected event, closing");
                    break;
                }
            }
        }
        info!("receive loop ended");
    }

    fn connection_send_loop(from_broker: Receiver<PeerMessage>, write_stream: &mut TcpStream) {
        info!(
            "sending loop started {:?}",
            write_stream.peer_addr().unwrap()
        );
        while let Ok(msg) = from_broker.recv() {
            match msg {
                PeerMessage::Message { content } => {
                    trace!("received msg from broker to forward");
                    if let Ok(_) = write_stream.write_all(content.as_bytes()) {
                        trace!("successfully sent");
                    } else {
                        warn!("unable to send");
                        break;
                    }
                }
                PeerMessage::Close => {
                    break;
                }
            }
        }
        info!("[  OK  ] sending loop ended");
    }

    fn broker_loop(broker_rx: Receiver<BrokerCmd>) {
        info!("broker is running");
        let mut peers: Vec<Sender<PeerMessage>> = Vec::new();

        while let Ok(msg) = broker_rx.recv() {
            match msg {
                BrokerCmd::NewPeer { sender } => {
                    debug!("new peer {:#?}", peers);
                    peers.push(sender);
                }
                BrokerCmd::Message { ref content } => {
                    debug!("forwarding to peers {:#?}", content);
                    for peer in &peers {
                        trace!("forwarding to peer {:#?}", peer);
                        if let Err(e) = peer.send(PeerMessage::Message {
                            content: content.into(),
                        }) {
                            warn!("peer is dead {}", e);
                        }
                    }
                }
            }
        }

        info!("broker is ended");
    }
}

struct StringMsgHandler;

impl MsgHandler for StringMsgHandler {}

const LOG_VAR: &str = "RUST_LOG";

fn main() {
    color_backtrace::install();
    if env::var(LOG_VAR).is_err() {
        env::set_var(LOG_VAR, "trace");
    }
    env_logger::init_from_env(Env::new().filter(LOG_VAR));

    if let Some(addr) = env::args().nth(1) {
        let handler = StringMsgHandler;

        let server = Server { addr, handler };
        server.start();
    } else {
        error!("please pass a listening address")
    }
}

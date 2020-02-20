class: middleish, center
### Rust Workshop Part 2
# Hands On
--

Ready your Laptops!

---
class: middle
## Agenda

### Build your own Chat

1. use the `std` library
2. printing
3. read stdin
4. networking
5. threads & channels
6. debug primitives
7. Serde


---
class: middle, inverse
# Step 0
### Minimal Command Line Application

---
.chapter[Step0]
## Read from arguments

--
```rust
// std::env::args();
pub fn args() -> Args { // Args is an Iterator<Item = String>
    ...
}
```

--
```rust
use std::env;

fn main() {
    // the the first element from the args iterator
    if let Some(addr) = env::args().nth(1) {
        println!("thanks for passing {:#?}", addr);

    } else {
        println!("please pass a sending address")
    }
}

```

---
.chapter[Step0]
## Read from Stdin (1/3)

```rust
use std::io;


    let mut buffer = String::new();

        // read one line from std into input
        io::stdin().read_line(&mut buffer)?;
        do_something(&buffer);
```
---
.chapter[Step0]
## Read from Stdin (2/3)

```rust
use std::io;


    let mut buffer = String::new();
    loop {
        // read one line from std into input
        io::stdin().read_line(&mut buffer)?;
        do_something(&buffer);


        buffer.clear();
    }
 
```

---
.chapter[Step0]
## Read from Stdin (3/3)

```rust
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    loop {
        // read one line from std into input
        io::stdin().read_line(&mut buffer)?;
        do_something(&buffer);

        if input == "exit\n" { break }
        buffer.clear();
    }
 
    Ok(())
}
```

---
class: middle
.chapter[take away]
## `std::io`
* `Write` & `Read` traits
* `std::io::BufReader`
* `std::io::BufWriter`
* iterators: `Lines`, `Bytes`, `Split`
* also: `Seek`

---
class: middle, inverse
# Step 1
### Sending on a Socket

---
class: middle
.chapter[Step1]
## TcpStream

```rust
use std::io::Write;

let mut stream: TcpStream = TcpStream::connect(addr)?;

// write_all is a method on the `Write` trait
stream.write_all(...)?
```

Implement a program that takes an address as argument and then sends every line you type to on that stream;

---
class: middle, inverse
# Step 2
### Receiving on a Socket

---
class: middleish
.chapter[Step2]
## std::net::TcpListener

```rust
let listener = TcpListener::bind(addr).unwrap();
listener.accept(); // Result<(TcpListener, SocketAddr)>
```

--
```rust
while let Ok((tcp_stream, remote)) = listener.accept() {
    // new connection
}
```

--
* `tcp_stream` implements `Read` and `Write`
* `io::BufReader`

---
class: middle
.chapter[Step2]
## Multiple Connections
```rust
thread::spawn(move || { // ...
```


---
class: middle, inverse
# Step 3

### Doing two things at once

---
class: middle
.chapter[Step3]
## hold my 🍺
--

<small>
```rust
fn recv_loop(addr: &str) {
    // ...
}
fn send_loop(addr: &str) {
    // ...
}
fn main() {
    match (
        env::args().nth(1).as_deref(),
        env::args().nth(2),
        env::args().nth(3),
    ) {
        (Some("send"), Some(addr), _) => send_loop(&addr),
        (Some("recv"), Some(addr), _) => recv_loop(&addr),
        (Some("both"),
        Some(send_addr),
        Some(recv_addr)
        ) => both_loops(&send_addr, &recv_addr),
        _ => {
            println!("please supply on of the following options");
            println!("$ send send_addr");
            println!("$ recv recv_addr");
            println!("$ both send_addr recv_addr");
        }
    }
}
```
</small>

---
class: middle
.chapter[Step3]
## better:

`TcpStream` can be cloned

--

```rust
if let Ok(read_stream) = TcpStream::connect(addr) {
    println!("Connection opened! {:?}.", addr);

    let write_stream = read_stream.try_clone().unwrap();
    thread::spawn(|| send_loop(write_stream));

```

---
class: middle, inverse
# Step 4

### Forwarding Server

---
class: middle
.chapter[Step4]

## layout

* loops / threads

* accept_loop
* connection_recv_loop
* connection_send_loop
* broker

---
class: middle
.chapter[Step4]
### accept loop
```rust
```
---

class: middle
.chapter[Step4]
### accept loop
```rust
```
---
class: middle
.chapter[Step4]
```rust
```
---
class: middle
.chapter[Step4]
```rust
```

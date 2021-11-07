// This code will listen  for an HTTP request at the address 127.0.0.1:3571 for incoming
// Tcp streams, respond, and render the HTML source file.

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3571").unwrap();
    // Ports 0 to 1023 are Well-Known Ports.
    // Ports 1024 to 49151 are Registered Ports (often registered by a software developer 
    // to designate a particular port for their application)
    // Ports 49152 to 65535 are Public Ports.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {

      let contents = fs::read_to_string("hello.html").unwrap();

      let response = format!( 
          "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
          contents.len(),
          contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

  } 
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!( 
          "{}\r\nContent-Length: {}\r\n\r\n{}",
          status_line,
          contents.len(),
          contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    
  }

    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


// Remember to stop the program by pressing ctrl-c 
// when you’re done running a particular version of the code. 
// Then restart: cargo run 'hello' after you’ve made each set of code changes
// to make sure you’re running the newest code.
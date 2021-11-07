// This code will listen at the address 127.0.0.1:3571 for incoming
// Tcp streams. When it gets an incoming stream, it will print
// "Connection established! "


use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3571").unwrap();
    // Ports 0 to 1023 are Well-Known Ports.
    // Ports 1024 to 49151 are Registered Ports (often registered by a software developer to designate a particular port for their application)
    // Ports 49152 to 65535 are Public Ports.

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}

// Remember to stop the program by pressing ctrl-c 
// when you’re done running a particular version of the code. 
// Then restart cargo run 'hello' after you’ve made each set of code changes
// to make sure you’re running the newest code.
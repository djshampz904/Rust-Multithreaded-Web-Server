use std::net::TcpListener;
use std_tcp::handle_connection;

// Code will listen to local address 127.0.0.1:7878
// Print connection establish when it gets incoming stream

fn main() {
    // initialize listener and provide end point
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

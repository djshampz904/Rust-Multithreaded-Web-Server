use std::net::TcpListener;
use std_tcp::handle_connection;
use std::thread;
use std_tcp::ThreadPool;

// Code will listen to local address 127.0.0.1:7878
// Print connection establish when it gets incoming stream

fn main() {
    // initialize listener and provide end point
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.as_ref().unwrap().execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

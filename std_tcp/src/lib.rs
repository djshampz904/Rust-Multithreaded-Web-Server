use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
    thread,
    sync::{ mpsc, Arc, Mutex }
};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
        let message = receiver.lock().unwrap().recv();

        match message {
            Ok(job) => {
                println!("Worker {id} got a job; executing.");

                job();
            }
            Err(_) => {
                println!("Worker {id} disconnected; shutting down.");
                break;
            }
        }

    });

        Worker {id, thread}
    }
}


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidInput(usize),

}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl Drop for ThreadPool {

    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
impl ThreadPool {

    // Create a new thread pool
    // Checks if size is greater than zero and if not panics
    // Otherwise create  a ThreadPool

    pub fn execute<F>(&self, f:F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        
        match size {
            0 => Err(PoolCreationError::InvalidInput(size)),
            _ => {
                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }
                Ok ( ThreadPool { workers, sender: Some(sender),})
            }
        }

    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\rContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}


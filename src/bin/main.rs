use std::{char::ToLowercase, error, fs, net::{TcpListener, TcpStream}, task::Context, 
thread::{self, Builder, Thread}, time::Duration};
use  std::io::prelude::*;
use client_server::ThreadPool;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:5050").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        let local_stream = stream.unwrap();

        // thread::spawn(f)

        pool.execute(||{
            handle_connection(local_stream);
        });
        
        

        // let local_stream = stream.unwrap();
        // println!("connetion established successfully!");

    }

}

pub fn handle_connection(mut stream: TcpStream){
    //declare a buffer to hold the client messages
    //initial value is one and holds 1024 byts (1KB) data
    // let mut buffer = [0; 1024];
    // // println!(
    // //     "Request: {}", 
    // //     //buffer[..] creates a slice that includes all elements of the buffer array
    // //     String::from_utf8_lossy(&mut buffer[..]),
        
    // // );
    
    // stream.read(&mut buffer).unwrap();

    // let mut contents =  fs::read_to_string("index copy.html").unwrap();
    
    // // stream.write(response.as_bytes()).unwrap();
    // // stream.flush();

    // if buffer.starts_with(b"GET / HTTP/1.1\r\n"){
    //     contents =  fs::read_to_string("index.html").unwrap();
    // }

    // let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", contents);
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get  = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_name) = {
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        }
        else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "index copy.html")
        }
    };

    let content = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{} \r\n Content length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}


 
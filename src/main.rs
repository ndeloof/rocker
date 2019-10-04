use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::str;


fn main() {
    let request = format!("GET /version HTTP/1.1\r\nHost: docker\r\n\r\n");

    println!("Hello Docker!");
    let mut stream = UnixStream::connect("/var/run/docker.sock").unwrap();
    println!("Connected to docker.sock");

    match stream.write_all(request.as_bytes()) {
        Ok(_) => println!("Wrote all data to socket"),
        Err(_) => println!("Ouch"),
    };

    const BUFFER_SIZE: usize = 1024;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut raw: Vec<u8> = Vec::new();
    loop {
        let len = match stream.read(&mut buffer) {
            Ok(len) => len,
            Err(_) => 0,
        };

        println!("Received {} bytes", len);

        for i in 0..len {
            raw.push(buffer[i]);
        }

        if len < BUFFER_SIZE {
            break;
        }
    }
    let response = str::from_utf8(&raw).unwrap();
    println!("Got answer: {}", response);
}

#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::io::{self, Error, Read, Write};
use std::net::IpAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::thread;

type Port = u16;

pub struct clientInfo {
    host: IpAddr,
    port: u16,
}

impl clientInfo {
    fn makeClient(&self) {
        let mut stream =
            TcpStream::connect((self.host, self.port)).unwrap_or_else(|error| std::process::exit(0));
        let mut input_stream = stream.try_clone().unwrap();

        let handler = thread::spawn(move || {
            let mut client_buffer = [0u8; 1024];

            loop {
                match input_stream.read(&mut client_buffer) {
                    Ok(n) => {
                        if n == 0 {
                            std::process::exit(0);
                        } else {
                            io::stdout().write(&client_buffer).unwrap();
                            io::stdout().flush().unwrap();
                        }
                    }
                    Err(error) => std::process::exit(-1),
                }
            }
        });

        let output_stream = &mut stream;
        let mut user_buffer = String::new();

        loop {
            io::stdin().read_line(&mut user_buffer).unwrap();

            Write::write_all(output_stream, user_buffer.as_bytes()).unwrap();
            output_stream.flush().unwrap();
        }
    }
}
fn main(){
    let client = clientInfo{
        host:"127.0.0.1".parse().expect("Fuck you."),
        port:42069
    };
    client.makeClient();
}

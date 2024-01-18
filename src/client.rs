#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::process;
use std::thread;
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::net::TcpListener;

type Port = u16;

pub fn makeClient() {
	let host = "127.0.0.1";
	let port = "40269";
	let mut stream = TcpStream::connect(
		(host.as_str(), port)
	).unwrap_or_else(|error|
		program.print_fail(error.to_string())
	);
	let mut input_stream = stream.try_clone().unwrap();

	let handler = thread::spawn(move || {
		let mut client_buffer = [0u8; 1024];

		loop {
			match input_stream.read(&mut client_buffer) {
				Ok(n) => {
					if n == 0 {
						program.exit(0);
					}
					else
					{
						io::stdout().write(&client_buffer).unwrap();
						io::stdout().flush().unwrap();
					}
				},
				Err(error) => program.print_fail(error.to_string()),
			}
		}
	});

	let output_stream = &mut stream;
	let mut user_buffer = String::new();

	loop {
		io::stdin().read_line(&mut user_buffer).unwrap();

		output_stream.write(user_buffer.as_bytes()).unwrap();
		output_stream.flush().unwrap();
	}
}
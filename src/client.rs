#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::io::{self, Error, Read, Write};
use std::net::IpAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::thread;
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::IpAddr;
type Port = u16;

struct Program {
	name: String
}

impl Program {
	fn new(name: String) -> Program {
		Program { name: name }
	}

	fn print_error(&self,mesg: String) {
		writeln!(io::stderr(),"{}: error: {}",self.name,mesg);
	}

	fn print_fail(&self,mesg: String) -> ! {
		self.print_error(mesg);
		self.fail();
	}

	fn exit(&self,status: i32) -> ! { process::exit(status) }
	fn fail(&self) -> ! { self.exit(-1); }
}

pub fn make_default_client() {
	let mut args = env::args();
	let program = Program::new(
		args.next().unwrap_or("test".to_string())
	);
	let host : IpAddr = "127.0.0.1"
	.parse()
	.expect("Unable to parse host");
	let port = 9999;

	let mut stream = TcpStream::connect(
		(host, port)
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

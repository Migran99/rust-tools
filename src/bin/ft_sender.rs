use std::env;
use migformatting::Formatting;

use std::net::TcpStream;
use std::fs::File;

use std::io::{copy};

fn main(){
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("{}",String::from("USE: <filename> <ip:port>").error());
        std::process::exit(0);
    }
    let filename = &arguments[1];
    let ip_port = &arguments[2];

    let mut myfile = File::open(filename).unwrap();
    let mut tcp_connection = TcpStream::connect(ip_port).unwrap();

    let n = copy(&mut myfile, &mut tcp_connection).unwrap();

    print!("{} bytes sent!", format!("{}",n).valid());

}

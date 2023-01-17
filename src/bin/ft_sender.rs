use std::env;
use formatting::Formatting;

use std::net::TcpStream;
use std::fs::File;

use std::io::{Read, Write, Result};

fn main() -> Result<()>{
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("{}",String::from("USE: <filename> <ip:port>").error());
        std::process::exit(0);
    }
    let filename = &arguments[1];
    let ip_port = &arguments[2];

    let mut myfile = File::open(filename).unwrap();
    let mut tcp_connection = TcpStream::connect(ip_port)?;

    let mut buf = [0; 4096];
    loop {
        let n = myfile.read(&mut buf)?;
        
        if n == 0 {
            // reached end of file
            break;
        }
        
        tcp_connection.write_all(&buf[..n])?;
    }
    
    Ok(())

}

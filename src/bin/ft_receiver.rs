use std::env;
use std::io;
use std::thread::sleep;
use std::time;
use formatting::Formatting;

use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

use std::io::{Result};

fn main() -> Result<()>{
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("{}",String::from("USE: <filename> <ip:port>").error());
        std::process::exit(0);
    }
    let filename = &arguments[1];
    let ip_port = &arguments[2];

    let mut myfile;
    let file_res = File::create(filename);
    match  file_res {
        Ok(f) => {myfile = f;},
        Err(e) => {panic!("{}",e);},
    }


    let tcp_connection = TcpListener::bind(ip_port).unwrap();

    let mut client : TcpStream = tcp_connection.accept().unwrap().0;

    

    loop {

        let mut buf = [0; 4096];
        let n = client.peek(&mut buf).unwrap();
    
        if n == 0 {
            sleep(time::Duration::from_millis(1000));
        }
        else {
            break;
        }
        
    } 


    let m = io::copy(&mut client, &mut myfile).unwrap();
    print!("{} bytes written!", format!("{}",m).valid());
    
    
    Ok(())

}

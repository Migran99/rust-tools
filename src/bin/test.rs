use std::env;
use text_file;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        println!("you need to give a filepath!!");
        std::process::exit(0);
    }
    let filename = &arguments[1];
    
    let mut my_file = text_file::TextFile::new(filename).unwrap();

    println!("File {} size in Bytes: {}", my_file.name(), my_file.len());

    my_file.print_content();
    my_file.apped_data("\nhello world");
    my_file.print_content();
}

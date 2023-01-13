use std::env;

use crate::formatting::Formatting;
mod text_file;
mod csv_file;
mod formatting;   

fn main() {
    use text_file::TextFile;
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        println!("you need to give a filepath!!");
        std::process::exit(0);
    }
    let filename = &arguments[1];
    
    let mut my_file = TextFile::new(filename).unwrap();

    println!("File {} size in Bytes: {}", my_file.name(), my_file.len());

    my_file.print_content();
    my_file.apped_data("\nhello world");
    my_file.print_content();

    use csv_file::CsvFile;

    let mut mycsv = CsvFile::new(filename);

    let keys = mycsv.get_keys();
    println!("{keys:?}");

    let keys_pairs= mycsv.get_key_pairs();
    println!("{keys_pairs:?}");

    let content = mycsv.get_content();
    println!("{content:?}");

    let mut new_content = ["example", "0"].map(|i| i.to_string()).to_vec();
    mycsv.add_entry(&new_content); // Invalid
    let content = mycsv.get_content();
    println!("{content:?}");

    new_content.push("3".to_string());
    mycsv.add_entry(&new_content); // Valid
    let content = mycsv.get_content();
    println!("{content:?}");

    println!("{}",format!("Done!").valid())
}

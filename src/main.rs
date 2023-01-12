use std::env;
mod text_file;
mod csv_file;   

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

    let mycsv = CsvFile::new(filename);

    let keys = mycsv.get_keys();
    println!("{keys:?}");

    let keys_pairs= mycsv.get_key_pairs();
    println!("{keys_pairs:?}");
}

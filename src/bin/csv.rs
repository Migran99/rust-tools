use csv_file::CsvFile;
use migformatting::Formatting;
use text_file;

use migparser::{ArgumentParser, DataType};

fn main() -> Result<(), String> {
    // Arguments configuration
    let mut parser = ArgumentParser::new();

    parser.add_argument("path", None, DataType::String, None, None)?;
    parser.parse_arguments();

    let filename = match parser.get_value::<String>("path") {
        Some(f) => f,
        None => {
            println!("{}", String::from("you need to give a filepath!!").error());
            std::process::exit(0);
        }
    };

    // CSV Example

    let mut my_file = text_file::TextFile::new(&filename).unwrap();

    println!("File {} size in Bytes: {}", my_file.name(), my_file.len());

    my_file.print_content();
    my_file.apped_data("\nhello world");
    my_file.print_content();

    let mut mycsv = CsvFile::new(&filename, None);

    let keys = mycsv.get_keys();
    println!("{keys:?}");

    let keys_pairs = mycsv.get_key_pairs();
    println!("{keys_pairs:?}");

    mycsv.display();

    let mut new_content = ["example", "0"].map(|i| i.to_string()).to_vec();
    mycsv.add_entry(&new_content); // Invalid
    mycsv.display();

    mycsv.set_name("Table A");

    new_content.push("3".to_string());
    mycsv.add_entry(&new_content); // Valid
    mycsv.display();

    mycsv.add_key("new_key");
    mycsv.display();

    mycsv.add_key("new_key");
    mycsv.display();

    let keys_pairs = mycsv.get_key_pairs();
    println!("{keys_pairs:?}");

    mycsv.set_key_value("new_key", 2, "test");

    mycsv.display();

    println!("{}", format!("Done!").valid());

    return Ok(());
}

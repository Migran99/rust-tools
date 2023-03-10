use migparser::ArgumentParser;
use migparser::DataType;
use migparser::ArgumentOptions;

fn main () {
    let mut parser = ArgumentParser::new();

    parser.add_argument("name", DataType::String, None, None);
    parser.add_argument("--number", DataType::Int, None, None);
    parser.add_argument("flag", DataType::Bool, 
            Some(vec![ArgumentOptions::STORE_TRUE, ArgumentOptions::NECESSARY]), None);
    parser.print_data();    

    parser.parse_arguments();
    parser.print_data();

    let name : String = parser.get_value("name").unwrap();
    println!("The name is {name}");

    match parser.get_value::<i32>("number") {
        Some(i) => {println!("The number is {i}");},
        None => {},
    }
    
}
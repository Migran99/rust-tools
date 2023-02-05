use migparser::ArgumentParser;
use migparser::ContentsTypes;
use migparser::ArgumentOptions;

fn main () {
    let mut parser = ArgumentParser::new();

    parser.add_argument("name", ContentsTypes::String, None, None);
    parser.add_argument("--number", ContentsTypes::Int, None, None);
    parser.add_argument("flag", ContentsTypes::Bool, 
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
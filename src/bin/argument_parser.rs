use argument_parser::ArgumentParser;
use argument_parser::ContentsTypes;
use argument_parser::ArgumentOptions;

fn main () {
    let mut parser = ArgumentParser::new();

    parser.add_argument("name", ContentsTypes::String, None, None);
    parser.add_argument("flag", ContentsTypes::Bool, 
            Some(vec![ArgumentOptions::STORE_TRUE, ArgumentOptions::NECESSARY]), None);
    parser.print_data();    

    parser.parse_arguments();
    parser.print_data();

    let name = parser.get_value::<String>("name").unwrap();
    println!("The name is {name}");
}
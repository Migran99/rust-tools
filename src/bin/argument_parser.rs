use argument_parser::ArgumentParser;
use argument_parser::ContentsTypes;

fn main () {
    let mut parser = ArgumentParser::new();

    parser.add_argument("name", ContentsTypes::Int, None);
    parser.add_argument("flag", ContentsTypes::Bool, Some(vec!["store-true"]));
    parser.print_data();    

    parser.parse_arguments();
    parser.print_data();
}
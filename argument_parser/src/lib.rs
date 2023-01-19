use std::{env};

// Supported types
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Contents {
    Char(char),
    Int(i32),
    Uint(u32),
    String(String),
    Bool(bool)
}
impl Contents {
    pub fn get_value_str(&self) -> String{
        match self {
            Contents::Bool(c) => format!("{c}"),
            Contents::Char(c) => format!("{c}"),
            Contents::Int(c) => format!("{c}"),
            Contents::Uint(c) => format!("{c}"),
            Contents::String(c) => c.clone(),

        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum ContentsTypes {
    Char,
    Int,
    Uint,
    String,
    Bool
}
#[derive(Debug)]
#[derive(Clone)]
struct Argument {
    name: String,
    data_type: ContentsTypes,
    data: Option<Contents>,
    options: Vec<String>,
}

pub struct ArgumentParser {
    arguments: Vec<Argument>,
}

impl ArgumentParser {

    // Instantiation
    pub fn new() -> ArgumentParser {
        ArgumentParser {arguments: vec![]}
    }

    // Arguments functions
    pub fn add_argument(&mut self, name: &str, data_type: ContentsTypes ,options_: Option<Vec<&str>>) {
        let mut data: Option<Contents> = None;
        if let Some(opt) = &options_ {
            if data_type == ContentsTypes::Bool
            {
                    if opt.iter().any(|&f| f == "store-true"){
                    data = Some(Contents::Bool(false));
                }
                else if opt.iter().any(|&f| f == "store-false") {
                    data = Some(Contents::Bool(true));
                }
            }
        }
        self.arguments.push(Argument {
            name: name.to_string(), 
            data_type,
            data: data,
            options: match options_ {
                Some(c) => c.iter().map(|&f| String::from(f)).collect(),
                None => vec![]
            }
        });
    }

    fn parse_text<T: std::str::FromStr>(text: &String) -> Option<T>{
        let parsed = text.parse::<T>().ok(); 
        match parsed {
            Some(c) => Some(c),
            None => None
        }
    }

    fn parse_value(text: &String, type_ : &ContentsTypes) -> Option<Contents> {
        let res: Option<Contents> = match type_ {
            ContentsTypes::Char => {
                let parsed = ArgumentParser::parse_text::<char>(text);
                if let Some(c) = parsed{
                    return Some(Contents::Char(c));
                }
                else {
                    return None;
                }
            },
            ContentsTypes::Int => {
                let parsed = ArgumentParser::parse_text::<i32>(text);
                if let Some(c) = parsed{
                    return Some(Contents::Int(c));
                }
                else {
                    return None;
                }
            },
            ContentsTypes::Uint => {
                let parsed = ArgumentParser::parse_text::<u32>(text);
                if let Some(c) = parsed{
                    return Some(Contents::Uint(c));
                }
                else {
                    return None;
                }
            },
            ContentsTypes::Bool => {
                let parsed = ArgumentParser::parse_text::<bool>(text);
                if let Some(c) = parsed{
                    return Some(Contents::Bool(c));
                }
                else {
                    return None;
                }
            },
            ContentsTypes::String => Some(Contents::String(text.clone())),
        };

        res
    }

    pub fn parse_arguments(&mut self) {
        let arguments: Vec<String> = env::args().collect();
        let mut used_arguments: Vec<bool> = vec![false; arguments.len()];

        for opt in self.arguments.iter_mut() {
            for (i,arg) in arguments.iter().enumerate() {
                if opt.name == *arg && !used_arguments[i] {
                    // Get value
                    used_arguments[i] = true;

                    if opt.options.iter().any(|f| f == "store-true") && opt.data_type == ContentsTypes::Bool{
                        opt.data = Some(Contents::Bool(true));
                    }
                    else if opt.options.iter().any(|f| f == "store-false") && opt.data_type == ContentsTypes::Bool {
                        opt.data = Some(Contents::Bool(false));
                    }
                    else {
                        let data = ArgumentParser::parse_value(&arguments[i+1], &opt.data_type);
                        opt.data = data;
                        used_arguments[i+1] = true;
                    }
                }
            }
        }
    }



    // Display functions
    pub fn print_data(&self) {
        println!("##### Arguments");
        for d in self.arguments.iter() {
            let data = if let Some(c) = &d.data {
                c.get_value_str()
            }
            else {
                "None".to_string()
            };
            println!("{:?} [{:?}] : {:?}", d.name, d.data_type, data);
        }
        println!("------");
    }
}
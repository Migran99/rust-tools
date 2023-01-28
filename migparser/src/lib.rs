use std::{env};
use migformatting::Formatting;

mod argument;
pub use argument::{ContentsTypes, ExtractFromContents, ArgumentOptions, Argument};
use argument::Contents;

pub struct ArgumentParser {
    arguments: Vec<Argument>,
}

impl ArgumentParser {

    // Instantiation
    pub fn new() -> ArgumentParser {
        ArgumentParser {arguments: vec![]}
    }

    // Arguments functions
    pub fn add_argument(&mut self, name: &str, data_type: ContentsTypes ,options_: Option<Vec<&str>>, default_value: Option<Contents>) {
        let mut data: Option<Contents> = default_value;
        if let Some(opt) = &options_ {
            if data_type == ContentsTypes::Bool
            {
                    if opt.iter().any(|&f| f == ArgumentOptions::STORE_TRUE){
                    data = Some(Contents::Bool(false));
                }
                else if opt.iter().any(|&f| f == ArgumentOptions::STORE_FALSE) {
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
            },
            parsed: false
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

                    if opt.has_option(ArgumentOptions::STORE_TRUE) && opt.data_type == ContentsTypes::Bool{
                        opt.data = Some(Contents::Bool(true));
                    }
                    else if opt.has_option(ArgumentOptions::STORE_FALSE) && opt.data_type == ContentsTypes::Bool {
                        opt.data = Some(Contents::Bool(false));
                    }
                    else {
                        let data = ArgumentParser::parse_value(&arguments[i+1], &opt.data_type);
                        opt.data = data;
                        used_arguments[i+1] = true;
                    }

                    opt.set_parsed();
                }
            }
            if !opt.is_parsed() && opt.has_option(ArgumentOptions::NECESSARY) {
                println!("{}", format!("Necessary argument '{}' is not present", opt.name).error());
                panic!();
            }
        }
    }

    pub fn get_value<T: ExtractFromContents>(&self, arg: &str) -> Option<T>{
        let mut ret: Option<Contents> = None;
        for a in self.arguments.iter() {
            if &a.name == arg {
                ret = a.get_data();
            }
        }

        match ret {
            Some(c) => c.get_value(),
            None => None
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
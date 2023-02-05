use std::{env};
use migformatting::Formatting;

mod argument;
pub use argument::{ContentsTypes, ExtractFromContents, ArgumentOptions, Argument};
use argument::{Contents, ArgumentType};

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
        let mut options = match options_ {
            Some(c) => c.iter().map(|&f| String::from(f)).collect(),
            None => vec![]
        };

        if data_type == ContentsTypes::Bool
        {
            if options.contains(&ArgumentOptions::STORE_TRUE.to_owned()){
                data = Some(Contents::Bool(false));
            }
            else if options.contains(&ArgumentOptions::STORE_FALSE.to_owned()) {
                data = Some(Contents::Bool(true));
            }
        }
        let argument_type = Argument::get_type(name);
        match argument_type {
            Some(t) => {
                match t {
                    ArgumentType::Optional => {
                        // Do nothing
                    },
                    ArgumentType::Postional => {
                        // Add the necesary option if not already
                        if !options.contains(&ArgumentOptions::NECESSARY.to_owned()) {
                                options.push(ArgumentOptions::NECESSARY.to_owned());
                        }

                    },
                }

            },
            None => {},
        }

        let arg_name = match Argument::parse_name(name) {
            Some(n) => { n },
            None => { name.into() },
        };
        let cl_name = name.to_owned(); // keeping --arg if present
        
        self.arguments.push(Argument {
            name: arg_name,
            cl_name, 
            data_type,
            data: data,
            options: options,
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
                if opt.cl_name == *arg && !used_arguments[i] {
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
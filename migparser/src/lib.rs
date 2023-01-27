use std::{env, fmt::Debug};
use migformatting::Formatting;

#[allow(non_snake_case)]
pub mod ArgumentOptions {
    pub static STORE_TRUE: &str = "STORE_TRUE";
    pub static STORE_FALSE: &str = "STORE_FALSE";
    pub static NECESSARY: &str = "NECESSARY";
}

// pub trait ContentStringConversion {
//     fn from_string(txt: &String) -> Option<Self> where Self: Sized;
//     // fn to_string(&self) -> String;
// }

// impl ContentStringConversion for String {
//     fn from_string(txt: &String) -> Option<String> {
//         Some(txt.clone())
//     }
//     // fn to_string(&self) -> String {
//     //     self.clone()
//     // }
// }
// impl ContentStringConversion for i32 {
//     fn from_string(txt: &String) -> Option<i32> {
//         txt.parse::<i32>().ok()
//     }
//     // fn to_string(&self) -> String {
//     //     format!("{self}")
//     // }
// }
// impl ContentStringConversion for u32 {
//     fn from_string(txt: &String) -> Option<u32> {
//         txt.parse::<u32>().ok()
//     }
//     // fn to_string(&self) -> String {
//     //     format!("{self}")
//     // }
// }
// impl ContentStringConversion for bool {
//     fn from_string(txt: &String) -> Option<bool> {
//         txt.parse::<bool>().ok()
//     }
//     // fn to_string(&self) -> String {
//     //     format!("{self}")
//     // }
// }

pub trait ExtractFromContents {
    fn extract(object: &Contents) -> Option<Self> where Self: Sized;
}

impl ExtractFromContents for i32 {
    fn extract(object: &Contents) -> Option<Self> {
        match object {
            Contents::Int(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for bool {
    fn extract(object: &Contents) -> Option<Self> {
        match object {
            Contents::Bool(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for u32 {
    fn extract(object: &Contents) -> Option<Self> {
        match object {
            Contents::Uint(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for String {
    fn extract(object: &Contents) -> Option<Self> {
        match object {
            Contents::String(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}


// Supported types
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Contents {
    Int(i32),
    Uint(u32),
    String(String),
    Bool(bool)
}
impl Contents {
    pub fn get_value_str(&self) -> String{
        match self {
            Contents::Bool(c) => c.to_string(),
            Contents::Int(c) => c.to_string(),
            Contents::Uint(c) => c.to_string(),
            Contents::String(c) => c.to_string(),
        }
    }
    pub fn get_value<T: ExtractFromContents>(&self) -> Option<T> {
        T::extract(self)
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum ContentsTypes {
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
    parsed: bool
}
impl Argument {
    pub fn get_data(&self) -> Option<Contents>{
        self.data.clone()
    }
    pub fn has_option(&self, option: &str) -> bool {
        self.options.iter().any(|f| f == option)
    }
    pub fn is_parsed(&self) -> bool{
        self.parsed.clone()
    }

    pub fn set_parsed(&mut self) {
        self.parsed = true;
    }
    
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
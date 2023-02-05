/// Implements content data structure and associated functionality.
/// 
/// The ExtractFromContents allows to directly extract the value of the enum
/// Any new types in Contents enum should also implement this trait.

// Supported types
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Contents {
    Int(i32),
    Uint(u32),
    String(String),
    Bool(bool),
    Float(f32)
}
impl Contents {
    pub fn get_value_str(&self) -> String{
        match self {
            Contents::Bool(c) => c.to_string(),
            Contents::Int(c) => c.to_string(),
            Contents::Uint(c) => c.to_string(),
            Contents::String(c) => c.to_string(),
            Contents::Float(c) => c.to_string(),
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

impl ExtractFromContents for f32 {
    fn extract(object: &Contents) -> Option<Self> {
        match object {
            Contents::Float(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}
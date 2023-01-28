mod contents;
pub use contents::{ContentsTypes, ExtractFromContents};
pub use contents::Contents;

#[allow(non_snake_case)]
pub mod ArgumentOptions {
    pub static STORE_TRUE: &str = "STORE_TRUE";
    pub static STORE_FALSE: &str = "STORE_FALSE";
    pub static NECESSARY: &str = "NECESSARY";
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub data_type: ContentsTypes,
    pub data: Option<Contents>,
    pub options: Vec<String>,
    pub parsed: bool
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
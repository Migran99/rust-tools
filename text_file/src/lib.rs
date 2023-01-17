
use std::fs;
use std::str;
pub struct TextFile {
    name: String,
    data: Vec<u8>,
}

impl TextFile {
    // UAX FUNCTIONS
    fn load(filename: &String) -> Result<Vec<u8>, &str> {
        /*
            Load a csv file
        */
        let file_fs = match fs::read(filename) {
            Ok(f) => f,
            Err(_) => return Err("Cannot load file"),
        };

        Ok(file_fs)
    }

    pub fn content_string(&self) -> &str {
        let s = match str::from_utf8(&self.data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 content: {}", e),
        };

        s
    }

    fn apped_bytes(&mut self, data: Vec<u8>) {
        let mut copy = data.clone();
        self.data.append(&mut copy);
    }

    // +++++++++++++++++++++++

    pub fn new(filename: &String) -> Result<TextFile, &str> {
        let data = match TextFile::load(&filename) {
            Ok(data) => data,
            Err(e) => {
                return Err(e);
            }
        };

        let file = TextFile {
            name: filename.clone(),
            data: data,
        };
        Ok(file)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn print_content(&self) {
        let content = self.content_string();
        println!("####################");

        for (_i, l) in content.lines().enumerate() {
            println!("{}", l);
        }
        println!("####################");
    }

    pub fn apped_data(&mut self, data: &str) {
        self.apped_bytes(data.as_bytes().to_vec());
    }
}



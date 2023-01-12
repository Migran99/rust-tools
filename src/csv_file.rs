use std::fs;
use std::str;

use super::text_file::TextFile;

pub struct CsvFile {
    txt_file: TextFile, // Vector of columns. Columns are vectors of strings.
    keys: Vec<String>,
    keys_pair: Vec<(String, u32)>
}

impl CsvFile {
    fn get_keys_txt(file: &TextFile) -> Vec<String> {
        let s = file.content_string();
        let first_line = s.lines().next().unwrap();
        let res = first_line.split(",").collect::<Vec<&str>>();

        res.iter().map(|&i| String::from(i)).collect()
    }


    pub fn new(filename: &String) -> CsvFile {
        let txt_file = TextFile::new(filename).unwrap();
        
        let keys = CsvFile::get_keys_txt(&txt_file);
        let mut keys_pair : Vec<(String, u32)> = [].to_vec();
        for (i,s) in keys.iter().enumerate(){
            keys_pair.push((s.clone(),u32::try_from(i).unwrap()));
        }

        CsvFile { txt_file: txt_file, keys: keys, keys_pair: keys_pair }

    }

    pub fn get_keys(&self) -> Vec<String>{
        self.keys.clone()
    }

    pub fn get_key_pairs(&self) -> Vec<(String, u32)> {
        self.keys_pair.clone()
    }


}
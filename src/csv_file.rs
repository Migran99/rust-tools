use std::fs;
use std::str;

use super::text_file::TextFile;

pub struct CsvFile {
    txt_file: TextFile, // Vector of columns. Columns are vectors of strings.
    keys: Vec<String>,
    keys_pair: Vec<(String, u32)>
}

impl CsvFile {
    fn getKeysTxt(file: &TextFile) -> Vec<String> {
        let s = file.content_string();
        let first_line = s.lines().next().unwrap();
        let res = first_line.split(",").collect::<Vec<&str>>();

        res.iter().map(|&i| String::from(i)).collect()
    }


    pub fn new(filename: &String) -> CsvFile {
        let txt_file = TextFile::new(filename).unwrap();
        
        let keys = CsvFile::getKeysTxt(&txt_file);
        let mut keys_pair : Vec<(String, u32)> = [].to_vec();
        for (i,s) in keys.iter().enumerate(){
            keys_pair.push((s.clone(),u32::try_from(i).unwrap()));
        }

        CsvFile { txt_file: txt_file, keys: keys, keys_pair: keys_pair }

    }

    pub fn getKeys(self) -> Vec<String>{
        self.keys.clone()
    }


}
use std::str;
use super::text_file::TextFile;
use super::formatting::Formatting;

pub struct CsvFile { 
    keys: Vec<String>,
    keys_pair: Vec<(String, u32)>,
    content: Vec<Vec<String>> // Vector of columns. Columns are vectors of strings.
}

impl CsvFile {
    fn get_keys_txt(file: &TextFile) -> Vec<String> {
        let s = file.content_string();
        let first_line = s.lines().next().unwrap();
        let res = first_line.split(",").collect::<Vec<&str>>();

        res.iter().map(|&i| String::from(i)).collect()
    }

    fn get_content_txt(file: &TextFile, keys : &Vec<String>) -> Vec<Vec<String>> {
        let lines_content = file.content_string().lines();
        let mut content : Vec<Vec<String>> = [].to_vec();
        for ( i , l ) in lines_content.enumerate() {
            if i > 0{
                let line_data = l.split(",").collect::<Vec<&str>>();
                if line_data.len() == keys.len(){
                    // Add the content if the number of elements is the same that the
                    // number of keys
                    content.push(line_data.iter().map(|&i| String::from(i.trim())).collect());

                }
            }
        }

        content
    }


    pub fn new(filename: &String) -> CsvFile {
        let txt_file = TextFile::new(filename).unwrap();
        
        let keys = CsvFile::get_keys_txt(&txt_file);
        let mut keys_pair : Vec<(String, u32)> = [].to_vec();
        for (i,s) in keys.iter().enumerate(){
            keys_pair.push((s.clone(),u32::try_from(i).unwrap()));
        }

        let content : Vec<Vec<String>> = CsvFile::get_content_txt(&txt_file, &keys);

        CsvFile { keys, keys_pair, content }

    }

    pub fn get_keys(&self) -> Vec<String>{
        self.keys.clone()
    }

    pub fn get_key_pairs(&self) -> Vec<(String, u32)> {
        self.keys_pair.clone()
    }

    pub fn get_content(&self) -> Vec<Vec<String>> {
        self.content.clone()
    }
    pub fn add_entry(&mut self, entry: &Vec<String>) {
        if entry.len() == self.keys.len() {
            self.content.push(entry.clone());
        }
        else {
            println!("{}",format!("Not a valid entry. Make sure it has the same number of keys as the csv table!").warning())
        }
    }


}
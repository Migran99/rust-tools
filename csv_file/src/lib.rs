use std::str;
use text_file::TextFile;
use formatting::Formatting;

pub struct CsvFile { 
    name: String,
    keys: Vec<String>,
    keys_pair: Vec<(String, usize)>,
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


    pub fn new(filename: &String, name: Option<String>) -> CsvFile {
        let txt_file = TextFile::new(filename).unwrap();

        
        let name = name.unwrap_or(String::from("untitled"));
        
        let keys = CsvFile::get_keys_txt(&txt_file);
        let mut keys_pair : Vec<(String, usize)> = [].to_vec();
        for (i,s) in keys.iter().enumerate(){
            keys_pair.push((s.clone(),i));
        }

        let content : Vec<Vec<String>> = CsvFile::get_content_txt(&txt_file, &keys);

        CsvFile { name, keys, keys_pair, content }

    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_keys(&self) -> Vec<String>{
        self.keys.clone()
    }

    pub fn get_key_pairs(&self) -> Vec<(String, usize)> {
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
            eprintln!("{}",format!("Not a valid entry. Make sure it has the same number of keys as the csv table!").warning())
        }
    }

    pub fn add_key(&mut self, key: &str) {
        if self.keys.iter().any(|i| i==key) {
            eprintln!("{}",format!("Error: Key already in the table.").error());
        }
        else {
            self.keys.push(String::from(key));
            let index = self.keys_pair.len();
            self.keys_pair.push((String::from(key), index));

            for i in self.content.iter_mut() {
                i.push("".to_string());
            }
        }
    }

    pub fn set_key_value(&mut self, key: &str, row: usize, content: &str) {
        if let None = self.keys.iter().find(|&i| i==key) {
            eprintln!("{}",format!("Error: Key not in the table.").error());
        }
        else if row >= self.content.len().try_into().unwrap(){
            eprintln!("{}",format!("Error: Row ID not in table.").error());
        }
        else {
            let index: usize = self.keys_pair.iter().position(|r| r.0 == key).unwrap();
            self.content[row][index] = String::from(content);
        }
    }

    pub fn display(&self) {
        println!("###### - {}", self.name);

        for (_i,k) in self.keys.iter().enumerate() {
            // if i != 0 {
            //     print!("{}",separator);
            // }
            // print!("{}", k);
            print!("{:>10}", k);
        }
        print!("\n");

        for (_i,k) in self.content.iter().enumerate() {
            for (_j, c) in k.iter().enumerate() {
                // if j != 0 {
                //     print!("{}",separator);
                // }
                // print!("{}", c);
                print!("{:>10}", c);
                
            } 
            print!("\n");
        }
        println!("######");
    }

}
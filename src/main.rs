use std::env;

mod csv{
    use std::fs;
    pub struct  CsvFile {
        name: String,
        data: Vec<u8>
    }
    
    impl CsvFile {
        fn load_csv(filename: &String) -> Result<Vec<u8>, &str>{
            /*
                Load a csv file
            */
            let file_fs = match fs::read(filename) {
                Ok(f) => {f},
                Err(_) => return Err("Cannot load file"),
            };
        
            Ok(file_fs)
        
        }
    
        pub fn new(filename: &String) -> Result<CsvFile, &str>{
            let data = match CsvFile::load_csv(&filename) {
                Ok(data) => {data},
                Err(e) => {return Err(e);},
            };

            let file = CsvFile{name: filename.clone(), data: data};

            Ok(file)

        }
    
        pub fn len(&self) -> usize{
            self.data.len()
        }
        pub fn name(&self) -> String{
            self.name.clone()
        }
        
    }

}    

fn main() {
    use csv::CsvFile;
    let arguments = env::args();
    let mut filename: String = "test.csv".to_string();
    for (index,argument) in arguments.enumerate(){
        if index != 0{
            println!("[{index}]{argument}");
            filename = argument;
        }
    }

    let my_file = match CsvFile::new(&filename) {
        Ok(my_file) => {my_file},
        Err(_) => {return;},
    };

    print!("File {} size in Bytes: {}", my_file.name(), my_file.len());
}

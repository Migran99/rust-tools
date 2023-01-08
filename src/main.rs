use std::env;

mod csv{
    use std::fs;
    use std::str;
    pub struct  CsvFile {
        name: String,
        data: Vec<u8>,
        metadata: Metadata
    }

    struct Metadata {
        item_number: u32,
    }
    
    impl CsvFile {

        // UAX FUNCTIONS
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

        fn content_string(&self) -> &str {
            let s = match str::from_utf8(&self.data) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 content: {}", e),
            };

            s
        }

        fn apped_bytes(&mut self, data: Vec<u8>){
            let mut copy = data.clone();
            self.data.append(&mut copy);
        }
        fn apped_data(&mut self, data: &str) {
            self.apped_bytes(data.as_bytes().to_vec());

        }

        fn get_item_number(&self) -> u32 {
            let content = self.content_string();
            let header = content.lines().next().unwrap();

            let header_items: Vec<&str> = header.split(',').collect(); 
            
            u32::try_from(header_items.len()).unwrap()
        }

        pub fn init(&mut self){
            self.metadata.item_number = self.get_item_number();
        }

        // +++++++++++++++++++++++
    
        pub fn new(filename: &String) -> Result<CsvFile, &str>{
            let data = match CsvFile::load_csv(&filename) {
                Ok(data) => {data},
                Err(e) => {return Err(e);},
            };

            let mut file = CsvFile{name: filename.clone(), data: data, metadata: Metadata { item_number: 0 }};
            file.init();


            Ok(file)

        }
    
        pub fn len(&self) -> usize{
            self.data.len()
        }
        pub fn name(&self) -> String{
            self.name.clone()
        }

        pub fn print_content(&self) {  
            let content = self.content_string();
            println!("####################");

            for (i,l) in content.lines().enumerate(){
                if i == 0
                    {println!("{}",l.to_uppercase());}
                else
                    {println!("{}",l);}
            }
            println!("####################");
        }

        pub fn add_line(&mut self, data: &mut [&str]) {
            self.apped_data("\n");

            for i in 0..self.metadata.item_number {
                if i < self.metadata.item_number-1 {
                    if i < data.len().try_into().unwrap() {
                        let a = data[usize::try_from(i).unwrap()];
                        self.apped_data(a);
                    }
                    self.apped_data(",");
                }
            }
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

    let mut my_file = CsvFile::new(&filename).unwrap();

    println!("File {} size in Bytes: {}", my_file.name(), my_file.len());

    my_file.print_content();
    my_file.add_line(&mut ["a"]);

    my_file.print_content();


}

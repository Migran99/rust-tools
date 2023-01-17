pub trait Formatting<T> {
    fn warning(&self) -> T;
    fn error(&self) -> T;
    fn valid(&self) -> T;
}

impl Formatting<String> for String {
    fn warning(&self) -> String{
        format!("\x1b[93m{}\x1b[0m", self.clone())
    }
    fn error(&self) -> String{
        format!("\x1b[91m{}\x1b[0m", self.clone())
    }
    fn valid(&self) -> String{
        format!("\x1b[92m{}\x1b[0m", self.clone())
    }
    
}
pub trait Formatting {
    fn warning(&self) -> String;
    fn error(&self) -> String;
    fn valid(&self) -> String;
}

impl Formatting for String {
    fn warning(&self) -> String{
        format!("\x1b[93m{}\x1b[0m", self.clone())
    }
    fn error(&self) -> String{
        format!("\x1b[91{}\x1b[0m", self.clone())
    }
    fn valid(&self) -> String{
        format!("\x1b[92m{}\x1b[0m", self.clone())
    }
    
}
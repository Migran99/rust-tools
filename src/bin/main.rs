use migformatting::Formatting;

fn main() {
    println!("{}{}", format!("EXECUTE ONE OF THE TESTS WITH: ").error(), format!("cargo run --bin <test>").warning());
    println!("{}{}", format!("To see a list of the tests: ").error(), format!("cargo run --bin>").warning());
}

mod cli;
fn main() {
    let value = cli::command();
    println!("{}", value);
}

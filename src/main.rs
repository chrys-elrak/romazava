mod cli;
mod emoji;
fn main() {
    let value = cli::command();
    println!("{}", value);
}

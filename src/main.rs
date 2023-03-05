mod cli;
mod romazava;
mod utils;
fn main() {
    let romazava = romazava::Romazava::new();
    let value = cli::command(romazava);
    println!("{}", value);
}

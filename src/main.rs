use clap::Parser;

mod args;

fn main() {
    let x = args::Args::parse();
    println!("{:?}", x.get_random_value());
}

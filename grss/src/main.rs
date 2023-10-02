#![allow(unused)]
use clap::Parser;

fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
}


#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

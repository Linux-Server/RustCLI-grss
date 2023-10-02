#![allow(unused)]
use clap::Parser;
use std::fs;


#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let data = fs::read_to_string("hello.txt");

    match data {
        Ok(content) => println!("{:?}", content),
        Err(err) => println!("{:?}", err)
        
    }

    // for line in data.lines(){
    //     if line.contains(&args.pattern){
    //         println!("{:?}", line);
    //     }
        
    // }
  
}



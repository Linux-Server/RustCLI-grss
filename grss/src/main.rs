#![allow(unused)]
use clap::Parser;
use std::fs;
use std::io;


#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main()-> Result<(), Box<dyn std::error::Error> >{
    let args = Cli::parse();
    let data = fs::read_to_string("hello1.txt")?;
    Ok(())
    

    // let result = match data {
    //     Ok(content) =>content,
    //     Err(err) => return Err(err.into())
        
    // };
    // Ok(())

    // for line in data.lines(){
    //     if line.contains(&args.pattern){
    //         println!("{:?}", line);
    //     }
        
    // }
    
  
}



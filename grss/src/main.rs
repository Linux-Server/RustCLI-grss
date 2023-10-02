
#![allow(unused)]
use clap::Parser;
use std::fs;
use std::io;
use anyhow::{Context, Result};


#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main()-> Result<()>{
    let args = Cli::parse();
    let data = fs::read_to_string("hellos.txt").with_context(|| format!("could not read file `{}`", "hello.txt"))?;
    // let data: String = fs::read_to_string("hello1.txt").map_err(|err| CustomError(format!("Error reading `{}`: {}", "hello.txt", err)))?;
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

#[derive(Debug)]
struct CustomError(String);





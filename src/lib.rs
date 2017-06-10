use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub input_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("please add input filename as a argument");
        }

        let input_filename = args[1].clone();

        Ok(Config {
            input_filename: input_filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.input_filename)?;

    let mut input = String::new();
    f.read_to_string(&mut input)?;

    println!("input:\n{}", input);

    Ok(())
}

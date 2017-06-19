extern crate time;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use time::get_time;

mod compute;
use compute::count_inc_seq;

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
    let start_time = get_time();
    let v: Vec<&str> = input.split('\n').collect();
    let N = v[0].parse::<u32>().expect("parse error");
    let mut start = 1;

    for i in 0..N {
        let parament: Vec<&str> = v[start].split(' ').collect();
        let n = parament[0].parse::<usize>().expect("parse error");
        let m = parament[1].parse::<usize>().expect("parse error");
        let x = parament[2].parse::<usize>().expect("parse error");
        let y = parament[3].parse::<usize>().expect("parse error");
        let z = parament[4].parse::<usize>().expect("parse error");
        let mut a: Vec<usize> = Vec::new();
        let mut s: Vec<usize> = Vec::new();
        for j in 0..m {
            start += 1;
            a.push(v[start].parse::<usize>().expect("parse error"));
        };
        for k in 0..n {
            s.push(a[k % m]);
            a[k % m] = (x * a[k % m] + y * (k + 1)) % z;
        };
        let answer: usize = count_inc_seq(s);
        println!("Case #{:?}: {:?}", i + 1, answer);
        start += 1;
    }
    let end_time = get_time();
    println!("running time: {:?}ms", (end_time - start_time).num_milliseconds());
    Ok(())
}

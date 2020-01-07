#![allow(dead_code)]
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod aoc10;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{}", &args[1]);
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", aoc03::solve_second(&contents));

    Ok(())
}

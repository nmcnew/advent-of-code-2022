
mod solutions;

use std::fs;
use clap::Parser;
use std::io::ErrorKind;
use crate::solutions::day_01;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.init {
        fs::create_dir("data")
            .expect("Couldn't create data path");
        let mut i = 1;
        while i <= 25 {
            let path = format!("data/day_{:02}.txt",i);
            let file_result = fs::write(path, "");
            match file_result {
                Ok(file) => file,
                Err(error) => {
                    match error.kind() {
                        ErrorKind::AlreadyExists => println!("{}File Already Exists, skipping", error),
                        _ => panic!("Unexpected error {}", error),
                    }                    
                },
            }
            i += 1;
        }
    }
    match args.day {
        1 => run_day_01(),
        _ => println!("Did not find a day command"),
    }
    Ok(())

    // let mut day = 1;
    // match day {
    //     1 => run_day_01(),
    //     _ => println!("No input?")
    // }
}

/// Advent of Code 2022 Runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of advent of code to run
    #[arg(short, long, default_value_t=1)]
    day: i32,
    /// Initialize the data files needed for advent of code
    #[arg(short, long, default_value_t=false)]
    init: bool,
}

fn run_day_01(){
    let values = fs::read_to_string("data/day_01.txt")
    .expect("Data file does not exist");
    let elf = day_01::get_packrat(values)
        .expect("This shouldn't be failing");
    println!("You should ask elf {} for some food, he has {} calories on him", elf.0, elf.1);
}
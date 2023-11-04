use std::env;
use std::fs;
use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.init {
        fs::create_dir("data")
            .expect("Couldn't create data path");
        let mut i = 1;
        while i <= 25 {
            let path = format!("data/day_{:02}.txt",i);
            fs::write(path, "")
                .expect("Couldn't write to given path");
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: i32,
    init: bool,
}

fn run_day_01(){
    fs::read_to_string("data/day_01.txt")
    .expect("Data file does not exist");
}
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use chrono::prelude::*;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "David M. Weis <dweis7@gmail.com>")]
struct Args {
    #[clap(short, long)]
    day: Option<u32>,
    #[clap(short, long)]
    all: bool,
}

fn main() {
    let args: Args = Args::parse();
    if args.all {
        day_1::run();
        println!();
        day_2::run();
        println!();
        day_3::run();
        println!();
        day_4::run();
        println!();
        day_5::run();
        println!();
        day_6::run();
        println!();
        day_7::run();
        println!();
        day_8::run();
        println!();
        day_9::run();
        return;
    }
    let day = if let Some(day) = args.day {
        day
    } else {
        let today: DateTime<Local> = Local::now();
        if today.month() != 12 {
            eprintln!("It's not december!\nYou probably want to specify a day with the `-d` arg");
            return;
        }
        today.day()
    };
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        7 => day_7::run(),
        8 => day_8::run(),
        9 => day_9::run(),
        _ => println!("No solution for that day yet!"),
    }
}

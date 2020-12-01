mod day_1;

use chrono::prelude::*;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "David M. Weis <dweis7@gmail.com>")]
struct Args {
    #[clap(short, long)]
    day: Option<u32>,
}

fn main() {
    let args: Args = Args::parse();
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
        _ => println!("No solution for that day yet!"),
    }
}

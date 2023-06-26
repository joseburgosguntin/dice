use clap::{arg, command, Parser};
use clap_num::number_range;
use rand::Rng;

fn in_side_range(s: &str) -> Result<u8, String> {
    number_range(s, 1, 6)
}

/// Simple program to roll a count of dice until they all are equal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which specific side must be repeated a count amount of times
    #[arg(short, long, value_parser=in_side_range)]
    side: Option<u8>,

    /// Max amount of rolls
    #[arg(short, long, default_value_t = 1_000_000)]
    max: u32,

    /// Number of dice rolled
    #[arg(short, long, default_value_t = 6)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let mut thread_rng = rand::thread_rng();
    let mut dice_rolled = 0;
    let mut generate_side = || {
        if dice_rolled == args.max {
            None
        } else {
            dice_rolled += 1;
            Some(thread_rng.gen_range(1..=6))
        }
    };

    let maybe = 'outer: loop {
        let Some(first) = generate_side() else {
            break None;
        };
        for _ in 0..args.count {
            let Some(other) = generate_side() else {
                break 'outer None;
            };
            if other != first {
                continue 'outer;
            }
        }
        match args.side {
            Some(side) if side == first => break Some(first),
            Some(_) => {}
            None => break Some(first),
        }
    };

    match maybe {
        Some(value) => println!(
            "Dice fell on {value} {} times after after rolling {dice_rolled} dice",
            args.count
        ),
        None => println!(
            "Dice failed to fall {} on a row on the same side, after rolling {dice_rolled} dice",
            args.count
        ),
    }
}

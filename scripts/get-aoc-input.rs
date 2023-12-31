#!/usr/bin/env -S cargo +nightly -Zscript

// This was originally written by Christ Biscardi: https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2023/rust/scripts/get-aoc-input.rs

//! ```cargo
//! [package]
//! edition = "2021"
//!
//! [dependencies]
//! clap = { version = "4.2", features = ["derive"] }
//! nom = "7.1.3"
//! reqwest = { version = "0.11.22", features=["blocking"] }
//! dotenv = "0.15.0"
//! ```

use clap::{error::ErrorKind, CommandFactory, Parser};
use nom::{bytes::complete::tag, character::complete, sequence::preceded, IResult};
use reqwest::{blocking::Client, header::COOKIE};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// day is expected to be formatted as
    /// `day-01` to match all other commands in
    /// the repo
    #[clap(short, long)]
    day: String,
    /// the aoc year we are doing
    #[clap(short, long)]
    year: u32,
    /// a way to pass in the justfile directory
    /// so that we're always in the root without
    /// doing any shenanigans
    #[clap(long)]
    current_working_directory: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32)(input)
}

fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let session = std::env::var("SESSION").expect("should have a session token set");
    let args = Args::parse();
    let Ok((_, day)) = parse_day(&args.day) else {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("day `{}` must be formatted as `day-01`", args.day),
        )
        .exit()
    };

    let year = args.year;
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    println!("sending to `{}`", url);

    let client = Client::new();
    let input_data = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;

    for filename in ["input1.txt", "input2.txt"] {
        let file_path = args
            .current_working_directory
            .join(&args.day)
            .join(filename);
        let mut file = File::create(&file_path).expect("should be able to create a file");

        file.write_all(input_data.as_bytes())
            .expect("should be able to write to input file");
        println!("wrote {}", file_path.display());
    }

    Ok(())
}

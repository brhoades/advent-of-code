mod year_2022;

use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::{bail, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "advent-of-code",
    about = "Runs advent of code solvers for a given year and problem provided an input."
)]
struct Opt {
    /// Year of advent of code problem to run.
    #[structopt(name = "year")]
    year: i32,

    /// The problem to run.
    #[structopt(name = "problem")]
    problem: i32,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(long = "log-level", short = "l")]
    log_level: Option<log::LevelFilter>,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opt = Opt::from_args();
    let input = read_to_string(opt.input).expect("failed to read input file");

    if let Some(lvl) = opt.log_level {
        log::set_max_level(lvl);
    }

    match (opt.year, opt.problem) {
        (2022, 5) => year_2022::day_05::run(input),
        (2022, 6) => year_2022::day_06::run(input),
        (2022, 7) => year_2022::day_07::run(input),
        (2022, 8) => year_2022::day_08::run(input),
        (2022, 9) => year_2022::day_09::run(input),
        (2022, 10) => year_2022::day_10::run(input),
        (2022, 11) => year_2022::day_11::run(input),
        (2022, 12) => year_2022::day_12::run(input),
        (2022, 13) => year_2022::day_13::run(input),
        (2022, 14) => year_2022::day_14::run(input),
        (2022, 15) => year_2022::day_15::run(input),
        (2022, 16) => year_2022::day_16::run(input),
        (2022, _) => bail!("unknown problem number {}", opt.problem),
        (_, _) => bail!("unkown year {}", opt.year),
    }
}

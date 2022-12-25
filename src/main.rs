mod twenty_two;

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
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let input = read_to_string(opt.input).expect("failed to read input file");

    match (opt.year, opt.problem) {
        (2022, 5) => twenty_two::five::run(input),
        (2022, 6) => twenty_two::six::run(input),
        (2022, 7) => twenty_two::seven::run(input),
        (2022, 8) => twenty_two::eight::run(input),
        (2022, 9) => twenty_two::nine::run(input),
        (2022, 10) => twenty_two::ten::run(input),
        (2022, _) => bail!("unknown problem number {}", opt.problem),
        (_, _) => bail!("unkown year {}", opt.year),
    }
}

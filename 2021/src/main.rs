mod challenges;

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // Day
    day: u8,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input_file: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    match args.day {
        1 => challenges::day1::solve(args.input_file),
        2 => challenges::day2::solve(args.input_file),
        3 => challenges::day3::solve(args.input_file),
        _ => println!("Problem for day {} not implemented", args.day)
    }
}

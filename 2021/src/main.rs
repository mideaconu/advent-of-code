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
}

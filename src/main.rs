//A command-line tool to predict weather alerts
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Alison",
    about = "A weather warning predictor"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "0.1.0", author = "Alison")]
    Predict {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Predict {}) => {
            let (_warn, _weath) = big_data::read_data();
            println!("Done!");
        }
        None => println!("Missing function parameter"),
    }
}

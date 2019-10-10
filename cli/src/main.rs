mod print;

use pipeline::from_ron;
use runner::Runner;
use std::{fs::read, path::PathBuf};
use structopt::StructOpt;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[derive(StructOpt)]
enum Cli {
    #[structopt(name = "print")]
    Print {
        #[structopt(default_value = "Pipeline.ron")]
        path: PathBuf,
        #[structopt(short = "v", long = "verbose")]
        verbose: bool,
    },
    #[structopt(name = "check")]
    Check {
        #[structopt(default_value = "Pipeline.ron")]
        path: PathBuf,
    },
    #[structopt(name = "run")]
    Run {
        #[structopt(default_value = "Pipeline.ron")]
        path: PathBuf,
    },
}

fn main() {
    let result = match Cli::from_args() {
        Cli::Print { path, verbose } => print_pipeline(path, verbose),
        Cli::Check { path } => check_pipeline(path),
        Cli::Run { path } => run_pipeline(path),
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}

fn print_pipeline(path: PathBuf, verbose: bool) -> Result {
    let pipeline = from_ron(&read(path)?)?;
    if verbose {
        print::verbose(&pipeline);
    } else {
        print::pretty(&pipeline, String::new());
    }
    Ok(())
}

fn check_pipeline(path: PathBuf) -> Result {
    from_ron(&read(path)?)?;
    println!("No errors");
    Ok(())
}

fn run_pipeline(path: PathBuf) -> Result {
    let mut runner = Runner::default();
    runner.add(from_ron(&read(path)?)?);

    runner.run_next();
    Ok(())
}

use std::fs::read;
use std::path::PathBuf;

use pipeline::from_toml;
use runner::Runner;
use structopt::StructOpt;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
enum Cli {
    #[structopt(name = "print")]
    Print {
        #[structopt(default_value = "Pipeline.toml")]
        path: PathBuf,
        #[structopt(short = "v", long = "verbose")]
        verbose: bool,
    },
    #[structopt(name = "check")]
    Check {
        #[structopt(default_value = "Pipeline.toml")]
        path: PathBuf,
    },
    #[structopt(name = "run")]
    Run {
        #[structopt(default_value = "Pipeline.toml")]
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
    let pipeline = from_toml(&read(path)?)?;
    if verbose {
        dbg!(pipeline);
    } else {
        for (idx, step) in pipeline.steps.iter().enumerate() {
            match &step.description {
                Some(description) => println!("Step {}: {}", idx + 1, description),
                None => println!("Step {}", idx + 1),
            }
        }
    }
    Ok(())
}

fn check_pipeline(path: PathBuf) -> Result {
    from_toml(&read(path)?)?;
    println!("No errors");
    Ok(())
}

fn run_pipeline(path: PathBuf) -> Result {
    let mut runner = Runner::default();
    runner.add(from_toml(&read(path)?)?);

    runner.run_next();
    Ok(())
}

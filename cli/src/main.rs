#![feature(process_exitcode_placeholder)]
mod print;

use job::Job;
use pipeline::from_ron;
use std::{fs::read, path::PathBuf, process::ExitCode};
use structopt::StructOpt;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[derive(StructOpt)]
#[structopt(name = "deploy cli")]
enum Cli {
    #[structopt(name = "print")]
    Print {
        #[structopt(default_value = "Deploy.ron")]
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

fn main() -> ExitCode {
    let result = match Cli::from_args() {
        Cli::Print { path, verbose } => print(path, verbose),
        Cli::Check { path } => check(path),
        Cli::Run { path } => run(path),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn print(path: PathBuf, verbose: bool) -> Result {
    let pipeline = from_ron(&read(path)?)?;
    if verbose {
        print::verbose(&pipeline);
    } else {
        print::pretty(&pipeline, String::new());
    }
    Ok(())
}

fn check(path: PathBuf) -> Result {
    from_ron(&read(path)?)?;
    println!("No errors");
    Ok(())
}

fn run(path: PathBuf) -> Result {
    Job::load(from_ron(&read(path)?)?, std::env::current_dir()?)?.run().map(|_| ()).or(Ok(()))
}

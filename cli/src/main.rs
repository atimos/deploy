mod print;

use job::Jobs;
use pipeline::from_ron;
use std::{fs::read, path::PathBuf};
use structopt::StructOpt;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[derive(StructOpt)]
enum Cli {
    #[structopt(name = "print")]
    Print {
        #[structopt(default_value = "Pipeline.ron")]
        path: PathBuf,
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
        Cli::Print { path } => print(path),
        Cli::Check { path } => check(path),
        Cli::Run { path } => run(path),
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}

fn print(path: PathBuf) -> Result {
    let pipeline = from_ron(&read(path)?)?;
    print::pretty(&pipeline, String::new());
    Ok(())
}

fn check(path: PathBuf) -> Result {
    from_ron(&read(path)?)?;
    println!("No errors");
    Ok(())
}

fn run(path: PathBuf) -> Result {
    let mut jobs = Jobs::new();
    jobs.append(from_ron(&read(path)?)?);

    let _ = jobs.next().unwrap().run();
    Ok(())
}

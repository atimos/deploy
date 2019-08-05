use std::fs::read;
use std::path::PathBuf;

//use job::from_pipeline;
use pipeline::from_toml;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cli {
    #[structopt(name = "print")]
    Print {
        #[structopt(default_value = "Pipeline.toml")]
        path: PathBuf,
        #[structopt(short = "b", long = "verbose")]
        verbose: bool,
    },
    #[structopt(name = "check")]
    Check {
        #[structopt(default_value = "Pipeline.toml")]
        path: PathBuf,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::Print { path , verbose } => print_pipeline(path, verbose),
        Cli::Check { path } => check_pipeline(path),
    }
}

fn print_pipeline(path: PathBuf, verbose: bool) {
    match read(path) {
        Ok(ref data) => match from_toml(data) {
            Ok(pipeline) => {
                if verbose {
                    dbg!(pipeline);
                } else {
                    for step in pipeline {
                        dbg!(step.description);
                    }
                }
            }
            Err(err) => println!("Error occured: {}", err),
        },
        Err(err) => println!("Error occured: {}", err),
    }
}

fn check_pipeline(path: PathBuf) {
    let mut count = 0;
    match read(&path) {
        Ok(ref data) => {
            for _ in 0..10000 {
                match from_toml(data) {
                    Ok(_pipeline) => {
                        count += 1;
                    }
                    Err(err) => println!("Error occured: {}", err),
                }
            }
        }
        Err(err) => println!("Error occured: {}", err),
    }

    dbg!(count);
}

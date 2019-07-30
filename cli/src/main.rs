use std::fs::read;
use std::path::PathBuf;

use job::from_pipeline;
use pipeline::from_toml;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cli {
    #[structopt(name = "print")]
    Print {
        #[structopt(default_value = "Pipeline.toml")]
        path: PathBuf,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::Print { path } => check_pipeline(path),
    }
}

fn check_pipeline(path: PathBuf) {
    match read(path) {
        Ok(ref data) => match from_toml(data) {
            Ok(pipeline) => {
                dbg!(from_pipeline(pipeline));
            }
            Err(err) => println!("Error occured: {}", err),
        },
        Err(err) => println!("Error occured: {}", err),
    }
}

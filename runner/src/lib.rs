use uuid::Uuid;
use std::path::PathBuf;
use pipeline::{Pipeline, Step, Commands, Command, Inline};

#[derive(Default)]
pub struct Runner {
    jobs: Vec<Job>,
    current: Option<Job>,
}

impl Runner {
    pub fn add(&mut self, pipeline: Pipeline) {
        let id = Uuid::new_v4();
        let job = Job {
            id: id,
            pipeline,
            space: Space {
                work: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref() + "/work"),
                publish: Publish {
                    html: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref() + "/publish/html"),
                    error: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref() + "/publish/error"),
                    log: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref() + "/publish/log"),
                },
                scratch: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref() + "/scratch"),
                artifact: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref() + "/artifact"),
            },
        };
        dbg!(&job.space);

        if self.current.is_none() {
            self.current = Some(job);
        } else {
            self.jobs.push(job);
        }
    }

    pub fn run_next(&mut self) {
        if let Some(mut job) = self.current.take() {
            //job.run();
        }
    }
}

#[derive(Debug)]
pub struct Job {
    pub id: Uuid,
    pub pipeline: Pipeline,
    pub space: Space,
}

impl Job {
    pub fn run(&mut self) {
        for step in &self.pipeline.steps {
            run_step(&step, &mut self.space);
        }
    }
}

#[derive(Debug)]
pub struct Space {
    pub work: PathBuf,
    pub publish: Publish,
    pub artifact: PathBuf,
    pub scratch: PathBuf,
}

#[derive(Debug)]
pub struct Publish {
    pub html: PathBuf,
    pub log: PathBuf,
    pub error: PathBuf,
}

fn run_step(step: &Step, space: &mut Space) {
    if let Some(cmd) = &step.run_before {
        run_command(cmd);
    }

    run_commands(&step.run);

    if let Some(cmd) = &step.run_after_error {
        run_command(cmd);
    }

    if let Some(cmd) = &step.run_after_success {
        run_command(cmd);
    }

    if let Some(cmd) = &step.run_after {
        run_command(cmd);
    }
}

fn run_commands(cmds: &Commands) {
    match cmds {
        Commands::Parallel(cmds) => {
            for cmd in cmds {
                run_command(cmd);
            }
        }
        Commands::SequenceRunAll(cmds) => {
            for cmd in cmds {
                run_command(cmd);
            }
        }
        Commands::SequenceStopOnError(cmds) => {
            for cmd in cmds {
                run_command(cmd);
            }
        }
    }
}

fn run_command(cmd: &Command) {
    if cmd.uri.scheme() == "inline" {
        //run_inline_unit(&cmd);
    } else {
        dbg!(cmd);
    }
}

fn run_inline_unit(unit: &Inline) {
    if let Some(cmd) = &unit.run_before {
        run_command(cmd);
    }

    run_commands(&unit.run);

    if let Some(cmd) = &unit.run_after_error {
        run_command(cmd);
    }

    if let Some(cmd) = &unit.run_after_success {
        run_command(cmd);
    }

    if let Some(cmd) = &unit.run_after {
        run_command(cmd);
    }
}

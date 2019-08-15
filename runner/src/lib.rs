use pipeline::{Pipeline, Units, Unit, Commands, Command, Arguments, ExecutionMode};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Default)]
pub struct Runner {
    jobs: Vec<Job>,
}

impl Runner {
    pub fn add(&mut self, pipeline: Pipeline) {
        let id = Uuid::new_v4();
        self.jobs.push(Job {
            id: id,
            pipeline,
            workspace: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref())
        });
    }

    pub fn run_next(&mut self) {
        if self.jobs.len() > 0 {
            self.jobs.remove(0).run();
        }
    }
}

#[derive(Debug)]
pub struct Job {
    pub id: Uuid,
    pub pipeline: Pipeline,
    pub workspace: PathBuf,
}

impl Job {
    pub fn run(&mut self) {
        for (idx, unit) in self.pipeline.steps.iter().enumerate() {
            println!("Running: Step {}: {:?}", idx, unit.description);
            run_unit(unit, &None, &self.pipeline.units);
        }
    }
}

fn run_unit(unit: &Unit, args: &Option<Arguments>, units: &Units) {
    println!("{:?}", args);
    run_cmds(&unit.commands, units);
}

fn run_cmds(cmds: &Commands, units: &Units) {
    match cmds {
        Commands::Multiple { commands, mode, .. } => {
            match mode {
                ExecutionMode::Parallel => run_in_parallel(&commands, units),
                ExecutionMode::SequenceRunAll => run_all_in_sequence(&commands, units),
                ExecutionMode::SequenceStopOnError => run_sequence_to_error(&commands, units),
            }
        }
        Commands::Single(cmd) => run_cmd(&cmd, units)
    }
}

fn run_in_parallel(list: &[Commands], units: &Units) {
    run_all_in_sequence(list, units);
}
fn run_all_in_sequence(list: &[Commands], units: &Units) {
    for cmds in list {
        run_cmds(cmds, units);
    }
}
fn run_sequence_to_error(list: &[Commands], units: &Units) {
    for cmds in list {
        run_cmds(cmds, units);
    }
}

fn run_cmd(cmd: &Command, units: &Units) {
    dbg!(cmd);
    match cmd {
        Command::Unit { id, args } => {
            run_unit(&units[id], args, units);
        }
        Command::Wasm { uri, command, args } => {
        }
        Command::Oci { repository, image, command, raw_command, force_rebuild, args } => {
        }
    }
}

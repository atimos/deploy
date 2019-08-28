mod environment;
mod program;
mod run;
mod template;

use derivative::Derivative;
use environment::{Environment, Status};
use pipeline::Pipeline;
use program::Programs;
use std::{convert::TryFrom, path::PathBuf};
use uuid::Uuid;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Runner {
    jobs: Vec<Job>,
}

impl Runner {
    pub fn add(&mut self, pipeline: Pipeline) {
        self.jobs.push(Job::load(pipeline, PathBuf::from(String::from("./deploy/"))));
    }

    pub fn run_next(&mut self) {
        if self.jobs.len() > 0 {
            self.jobs.remove(0).run();
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Job {
    instance_id: Uuid,
    pipeline: Pipeline,
    programs: Programs,
    environment: Environment,
}

impl<'a> Job {
    pub fn load(pipeline: Pipeline, workspace: PathBuf) -> Self {
        Self {
            instance_id: Uuid::new_v4(),
            programs: Programs::try_from((&pipeline, workspace.as_ref())).unwrap(),
            pipeline,
            environment: Environment {
                status: Status::Success,
                config: None,
                unit: None,
                instance: None,
                workspace,
            },
        }
    }

    pub fn run(&self) {
        run::pipeline(&self.pipeline, Vec::new(), &self.programs, self.environment.clone());
    }
}

mod environment;
mod program;

use pipeline::InstanceId;
use pipeline::Pipeline;
use program::Programs;
use std::convert::TryFrom;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Runner {
    jobs: Vec<Job>,
}

impl Runner {
    pub fn add(&mut self, pipeline: Pipeline) {
        self.jobs.push(Job::load(
            pipeline,
            PathBuf::from(String::from("./deploy/")),
        ));
    }

    pub fn run_next(&mut self) {
        if self.jobs.len() > 0 {
            self.jobs.remove(0).run();
        }
    }
}

#[derive(Debug)]
pub struct Job {
    instance_id: Uuid,
    pipeline: Pipeline,
    programs: Programs,
    workspace: PathBuf,
}

impl<'a> Job {
    pub fn load(pipeline: Pipeline, workspace: PathBuf) -> Self {
        Self {
            instance_id: Uuid::new_v4(),
            programs: Programs::try_from(&pipeline).unwrap(),
            pipeline,
            workspace,
        }
    }
    pub fn run(&mut self) {
        self.run_pipeline(&self.pipeline, Vec::new());
    }

    fn run_pipeline(&self, pipeline: &Pipeline, mut ids: Vec<InstanceId>) {
        match pipeline {
            Pipeline::List {
                list, instance_id, ..
            } => {
                ids.push(instance_id.clone());
                for pipeline in list {
                    self.run_pipeline(pipeline, ids.clone())
                }
            }
            Pipeline::On {
                condition,
                on_success,
                on_error,
                on_abort,
                instance_id,
                ..
            } => {
                ids.push(instance_id.clone());
                self.run_pipeline(condition, ids.clone());

                if let Some(pipeline) = on_success {
                    self.run_pipeline(pipeline, ids.clone());
                }
                if let Some(pipeline) = on_error {
                    self.run_pipeline(pipeline, ids.clone());
                }

                if let Some(pipeline) = on_abort {
                    self.run_pipeline(pipeline, ids.clone());
                }
            }
            Pipeline::Program {
                cmds,
                args,
                instance_id,
                ..
            } => {
                ids.push(instance_id.clone());
                self.programs.run(&ids, cmds, args);
            }
        }
    }
}

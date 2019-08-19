mod environment;
//mod program;
//mod oci;
//mod unit;
//mod wasm;

use pipeline::Pipeline;
use std::path::PathBuf;
use uuid::Uuid;

trait Program {
    fn load(self) -> Self;
    fn run(&self);
}

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
    //programs: Programs,
    workspace: PathBuf,
}

impl Job {
    pub fn load(pipeline: Pipeline, workspace: PathBuf) -> Self {
        Self {
            instance_id: Uuid::new_v4(),
            //programs: Programs::from(&pipeline),
            pipeline,
            workspace,
        }
    }
    pub fn run(&mut self) {
        // let mut env = environment::Environment {
        //     status: environment::Status::Success,
        //     config: None,
        //     unit: None,
        //     instance: None,
        //     workspace: self.workspace,
        // };

        // for (idx, unit) in self.pipeline.steps.iter().enumerate() {
        //     println!("Running: Step {}: {:?}", idx, unit.description);
        //     unit::run(unit, &None, &self.pipeline.units, &mut env, InstanceIds::new());
        // }
    }
}

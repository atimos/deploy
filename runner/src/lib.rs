mod environment;
mod oci;
mod unit;
mod wasm;

use pipeline::Pipeline;
use std::path::PathBuf;
use uuid::Uuid;
use pipeline::InstanceId;

#[derive(Default)]
pub struct Runner {
    jobs: Vec<Job>,
}

type InstanceIds<'a> = Vec<&'a InstanceId>;

impl Runner {
    pub fn add(&mut self, pipeline: Pipeline) {
        let id = Uuid::new_v4();
        self.jobs.push(Job {
            id: id,
            pipeline,
            workspace: PathBuf::from(String::from("./deploy/") + id.to_string().as_ref()),
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
        let mut env = environment::Environment {
            status: environment::Status::Success,
            config: None,
            unit: None,
            instance: None,
            workspace: PathBuf::from("./deploy"),
        };

        let scripts = wasm::prepare(&self.pipeline);
        let containers = oci::prepare(&self.pipeline);
        dbg!(scripts);
        dbg!(containers);

        // for (idx, unit) in self.pipeline.steps.iter().enumerate() {
        //     println!("Running: Step {}: {:?}", idx, unit.description);
        //     unit::run(unit, &None, &self.pipeline.units, &mut env, InstanceIds::new());
        // }
    }
}

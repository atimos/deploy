mod environment;
mod oci;
mod unit;
mod wasm;

use pipeline::Pipeline;
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
        for (idx, unit) in self.pipeline.steps.iter().enumerate() {
            println!("Running: Step {}: {:?}", idx, unit.description);
            unit::run(unit, &None, &self.pipeline.units, &mut Default::default());
            println!("");
        }
    }
}

mod program;

use pipeline::Node;
use program::Programs;

#[derive(Default)]
pub struct Jobs(Vec<Job>);

impl Jobs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&mut self, pipeline: Node) {
        self.0.push(Job::new(pipeline));
    }
}

impl Iterator for Jobs {
    type Item = Job;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

pub struct Job {
    pipeline: Node,
    programs: Programs,
}

impl Job {
    fn new(pipeline: Node) -> Self {
        Self { programs: Programs::new(&pipeline), pipeline }
    }

    pub fn run(self) -> Result<(), ()> {
        Ok(())
    }
}

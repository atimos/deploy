mod environment;
mod error;
mod program;

use environment::Environment;
use pipeline::Node;
use program::Programs;
use std::path::PathBuf;

#[derive(Default)]
pub struct Jobs(Vec<Job>);
type Result<T> = std::result::Result<T, error::Error>;

impl Jobs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn load(&mut self, pipeline: Node, path: PathBuf) -> Result<()> {
        self.0.push(Job::new(pipeline, path)?);
        Ok(())
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
    environment: Environment,
    programs: Programs,
}

impl Job {
    fn new(pipeline: Node, path: PathBuf) -> Result<Self> {
        Ok(Self {
            environment: Environment::new(path),
            programs: Programs::new(&pipeline)?,
            pipeline,
        })
    }

    pub fn run(self) -> Result<()> {
        run_node(self.pipeline, &self.programs)
    }
}

fn run_node(node: Node, programs: &Programs) -> Result<()> {
    match node {
        Node::Program { id, commands, arguments, .. } => {
            programs.run(&id, &arguments.map(Into::into), &commands)
        }
        Node::List { list, .. } => list.into_iter().map(|node| run_node(node, programs)).collect(),
    }
}

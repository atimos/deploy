mod environment;
mod program;

use environment::Environment;
use pipeline::Node;
use program::Programs;
use std::path::Path;

#[derive(Default)]
pub struct Jobs<'a>(Vec<Job<'a>>);

impl<'a> Jobs<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&mut self, pipeline: Node, path: &'a Path) {
        self.0.push(Job::new(pipeline, path));
    }
}

impl<'a> Iterator for Jobs<'a> {
    type Item = Job<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

pub struct Job<'a> {
    pipeline: Node,
    environment: Environment<'a>,
    programs: Programs,
}

impl<'a> Job<'a> {
    fn new(pipeline: Node, path: &'a Path) -> Self {
        Self { environment: Environment::new(path), programs: Programs::new(&pipeline), pipeline }
    }

    pub fn run(self) -> Result<(), ()> {
        run_node(&self.pipeline, &self.programs)
    }
}

fn run_node(node: &Node, programs: &Programs) -> Result<(), ()> {
    match node {
        Node::Program { id, commands, arguments, .. } => programs.run(id, arguments, commands),
        Node::List { list, .. } => list.iter().map(|node| run_node(node, programs)).collect(),
    }
}

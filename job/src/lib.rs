mod environment;
mod error;
mod program;

use environment::Environment;
use pipeline::{InstanceId, Node};
use program::{Output, Programs};
use std::{collections::HashMap, path::PathBuf};

pub struct Job {
    pipeline: Node,
    programs: Programs,
    environment: Environment,
}

pub type OutputMap = HashMap<InstanceId, Result<Vec<Output>, error::Error>>;

impl Job {
    pub fn load(pipeline: Node, workspace: PathBuf) -> Result<Self, error::Error> {
        Ok(Self {
            environment: Environment::new(workspace),
            programs: Programs::load(&pipeline)?,
            pipeline,
        })
    }

    pub fn run(mut self) -> Result<(Node, OutputMap), (Node, OutputMap)> {
        match run_node(&self.pipeline, &self.programs, &mut self.environment.vars) {
            Ok(output) => Ok((self.pipeline, dbg!(output))),
            Err(output) => Err((self.pipeline, dbg!(output))),
        }
    }
}

fn run_node(
    node: &Node,
    programs: &Programs,
    environment: &mut Variables,
) -> Result<OutputMap, OutputMap> {
    match node {
        Node::Commands { id, commands, local, .. } => {
            match programs.run(id, commands, environment) {
                Ok(out) => Ok(vec![(id.to_owned(), Ok(out))].into_iter().collect()),
                Err(out) => Err(vec![(id.to_owned(), Err(out))].into_iter().collect()),
            }
        }
        Node::Nodes { nodes, .. } => {
            let mut output = OutputMap::with_capacity(programs.len());

            for result in nodes.iter().map(|node| run_node(node, programs, environment)) {
                match result {
                    Ok(out) => {
                        output = output.into_iter().chain(out.into_iter()).collect();
                    }
                    Err(out) => return Err(output.into_iter().chain(out.into_iter()).collect()),
                }
            }
            Ok(output)
        }
    }
}

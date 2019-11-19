mod environment;
mod error;
mod program;

use environment::Environment;
use pipeline::{InstanceId, Node};
use program::Programs;
use std::{collections::HashMap, path::PathBuf, result::Result as StdResult};

pub struct Job {
    pipeline: Node,
    // programs: Programs,
    environment: Environment,
}

pub type Result = StdResult<(Node, HashMap<InstanceId, StdResult<(), ()>>), (Node, error::Error)>;

impl Job {
    pub fn load(pipeline: Node, workspace: PathBuf) -> StdResult<Self, error::Error> {
        Ok(Self {
            environment: Environment::new(workspace),
            // programs: Programs::load(&pipeline)?,
            pipeline,
        })
    }

    pub fn run(mut self) -> StdResult<(), ((), error::Error)> {
        Ok(())
        // let result = HashMap::default();
        //     match run_node(&self.pipeline, &self.programs, &mut
        // self.environment, &mut result) {         Ok(_) => Ok(Result {
        //             pipeline: self.pipeline,
        //             run_result: HashMap::default(),
        //             environment: self.environment,
        //         }),
        //         Err(err) => Err((
        //             Result {
        //                 pipeline: self.pipeline,
        //                 run_result: HashMap::default(),
        //                 environment: self.environment,
        //             },
        //             err,
        //         )),
        //     }
    }
}

// fn run_node(
//     node: &Node,
//     programs: &Programs,
//     environment: &mut Environment,
//     result: Result,
// ) -> StdResult<(), error::Error> {
//     Ok(())
//     // match node {
//     //     Node::Commands { id, commands, arguments, .. } => {
//     //         programs.run(&id, &commands, &arguments.map(Into::into), &mut
//     // env)     }
//     //     Node::List { list, mode, .. } => match mode {
//     //         ExecutionMode::Sequence => {
//     //             for node in list {
//     //                 run_node(node, env.clone(), programs)?;
//     //             }
//     //             Ok(())
//     //         }
//     //         ExecutionMode::Parallel => {
//     //             list.into_iter().map(|node| run_node(node, env.clone(),
//     // programs)).collect()         }
//     //     },
//     // }
// }

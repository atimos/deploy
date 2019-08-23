mod environment;
mod program;

use environment::{Environment, Status};
use pipeline::Arguments;
use pipeline::Command;
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
                workspace: workspace,
            },
        }
    }
    pub fn run(&self) {
        run_pipeline(
            &self.pipeline,
            Vec::new(),
            &self.programs,
            self.environment.clone(),
        );
    }
}

fn run_pipeline(
    pipeline: &Pipeline,
    mut id: Vec<InstanceId>,
    programs: &Programs,
    env: Environment,
) -> Result<Environment, Environment> {
    match pipeline {
        Pipeline::List {
            list, instance_id, ..
        } => {
            id.push(instance_id.clone());
            run_list(id, programs, env, &list)
        }
        Pipeline::On {
            cond,
            success,
            error,
            abort,
            instance_id,
            ..
        } => {
            id.push(instance_id.clone());
            run_condition(id, programs, env, cond, success, error, abort)
        }
        Pipeline::Program {
            cmds,
            args,
            instance_id,
            ..
        } => {
            id.push(instance_id.clone());
            run_program(id, programs, env, cmds, args)
        }
    }
}

fn run_list(
    id: Vec<InstanceId>,
    programs: &Programs,
    mut env: Environment,
    list: &[Pipeline],
) -> Result<Environment, Environment> {
    for pipeline in list {
        match run_pipeline(pipeline, id.clone(), programs, env) {
            Ok(new_env) | Err(new_env) => env = new_env,
        }
    }
    Ok(env)
}

fn run_condition(
    id: Vec<InstanceId>,
    programs: &Programs,
    env: Environment,
    cond: &Pipeline,
    success: &Option<Box<Pipeline>>,
    error: &Option<Box<Pipeline>>,
    abort: &Option<Box<Pipeline>>,
) -> Result<Environment, Environment> {
    run_pipeline(cond, id.clone(), programs, env.clone());

    if let Some(pipeline) = success {
        run_pipeline(pipeline, id.clone(), programs, env.clone());
    }
    if let Some(pipeline) = error {
        run_pipeline(pipeline, id.clone(), programs, env.clone());
    }

    if let Some(pipeline) = abort {
        run_pipeline(pipeline, id.clone(), programs, env.clone());
    }
    Ok(env)
}

fn run_program(
    id: Vec<InstanceId>,
    programs: &Programs,
    env: Environment,
    cmds: &[Command],
    args: &Arguments,
) -> Result<Environment, Environment> {
    programs.run(&id, cmds, args, env.clone());
    Ok(env)
}

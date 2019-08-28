use crate::{environment::Environment, program::Programs};
use pipeline::{Arguments, Command, InstanceId, Pipeline};

pub enum Error {}

pub fn pipeline(
    pipeline: &Pipeline,
    mut id: Vec<InstanceId>,
    programs: &Programs,
    env: Environment,
) -> Result<Environment, (Error, Environment)> {
    match pipeline {
        Pipeline::List { list, instance_id, .. } => {
            id.push(instance_id.clone());
            self::list(id, programs, env, &list)
        }
        Pipeline::On { cond, success, error, abort, instance_id, .. } => {
            id.push(instance_id.clone());
            condition(id, programs, env, cond, success, error, abort)
        }
        Pipeline::Program { cmds, args, instance_id, .. } => {
            id.push(instance_id.clone());
            program(id, programs, env, cmds, args)
        }
    }
}

fn list(
    id: Vec<InstanceId>,
    programs: &Programs,
    mut env: Environment,
    list: &[Pipeline],
) -> Result<Environment, (Error, Environment)> {
    for pipeline in list {
        match self::pipeline(pipeline, id.clone(), programs, env) {
            Ok(new_env) | Err((_, new_env)) => env = new_env,
        }
    }
    Ok(env)
}

fn condition(
    id: Vec<InstanceId>,
    programs: &Programs,
    env: Environment,
    cond: &Pipeline,
    success: &Option<Box<Pipeline>>,
    error: &Option<Box<Pipeline>>,
    abort: &Option<Box<Pipeline>>,
) -> Result<Environment, (Error, Environment)> {
    pipeline(cond, id.clone(), programs, env.clone());

    if let Some(pipeline) = success {
        self::pipeline(pipeline, id.clone(), programs, env.clone());
    }
    if let Some(pipeline) = error {
        self::pipeline(pipeline, id.clone(), programs, env.clone());
    }

    if let Some(pipeline) = abort {
        self::pipeline(pipeline, id.clone(), programs, env.clone());
    }
    Ok(env)
}

fn program(
    id: Vec<InstanceId>,
    programs: &Programs,
    env: Environment,
    cmds: &[Command],
    args: &Arguments,
) -> Result<Environment, (Error, Environment)> {
    programs.run(&id, cmds, args, env.clone());
    Ok(env)
}

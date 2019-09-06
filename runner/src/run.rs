use crate::{environment::Environment, program::Programs};
use pipeline::{Arguments, Command, Id, Pipeline};

pub enum Error {}

pub fn pipeline(
    pipeline: &Pipeline,
    mut ids: Vec<Id>,
    programs: &Programs,
    env: Environment,
) -> Result<Environment, (Error, Environment)> {
    match pipeline {
        Pipeline::List { list, id, .. } => {
            ids.push(id.clone());
            self::list(ids, programs, env, &list)
        }
        Pipeline::On { cond, success, error, abort, id, .. } => {
            ids.push(id.clone());
            condition(ids, programs, env, cond, success, error, abort)
        }
        Pipeline::Program { cmds, args, id, .. } => {
            ids.push(id.clone());
            program(ids, programs, env, cmds, args)
        }
    }
}

fn list(
    ids: Vec<Id>,
    programs: &Programs,
    mut env: Environment,
    list: &[Pipeline],
) -> Result<Environment, (Error, Environment)> {
    for pipeline in list {
        match self::pipeline(pipeline, ids.clone(), programs, env) {
            Ok(new_env) | Err((_, new_env)) => env = new_env,
        }
    }
    Ok(env)
}

fn condition(
    ids: Vec<Id>,
    programs: &Programs,
    env: Environment,
    cond: &Pipeline,
    success: &Option<Box<Pipeline>>,
    error: &Option<Box<Pipeline>>,
    abort: &Option<Box<Pipeline>>,
) -> Result<Environment, (Error, Environment)> {
    pipeline(cond, ids.clone(), programs, env.clone());

    if let Some(pipeline) = success {
        self::pipeline(pipeline, ids.clone(), programs, env.clone());
    }
    if let Some(pipeline) = error {
        self::pipeline(pipeline, ids.clone(), programs, env.clone());
    }

    if let Some(pipeline) = abort {
        self::pipeline(pipeline, ids.clone(), programs, env.clone());
    }
    Ok(env)
}

fn program(
    ids: Vec<Id>,
    programs: &Programs,
    env: Environment,
    cmds: &[Command],
    args: &Option<Arguments>,
) -> Result<Environment, (Error, Environment)> {
    programs.run(&ids, cmds, args, env.clone());
    Ok(env)
}

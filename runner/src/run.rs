use crate::{environment::Environment, program::Programs};
use pipeline::{Arguments, Block, Command, InstanceId};

pub enum Error {}

pub type Result = std::result::Result<Environment, (Error, Environment)>;
pub type Args = Option<Arguments>;

pub fn block(block: &Block, programs: &Programs, env: Environment) -> Result {
    match block {
        Block::List { list, .. } => self::list(programs, env, &list),
        Block::On { condition, success, error, abort, .. } => {
            self::condition(programs, env, condition, success, error, abort)
        }
        Block::Commands { commands, arguments, id, .. } => {
            self::command(id, programs, env, commands, arguments)
        }
    }
}

fn list(programs: &Programs, mut env: Environment, list: &[Block]) -> Result {
    for block in list {
        match self::block(block, programs, env) {
            Ok(new_env) | Err((_, new_env)) => env = new_env,
        }
    }
    Ok(env)
}

fn condition(
    programs: &Programs,
    env: Environment,
    cond: &Block,
    success: &Option<Box<Block>>,
    error: &Option<Box<Block>>,
    abort: &Option<Box<Block>>,
) -> Result {
    block(cond, programs, env.clone());

    if let Some(block) = success {
        self::block(block, programs, env.clone());
    }
    if let Some(block) = error {
        self::block(block, programs, env.clone());
    }

    if let Some(block) = abort {
        self::block(block, programs, env.clone());
    }
    Ok(env)
}

fn command(
    id: &InstanceId,
    programs: &Programs,
    env: Environment,
    cmds: &[Command],
    args: &Args,
) -> Result {
    programs.run(id, cmds, args, env.clone());
    Ok(env)
}

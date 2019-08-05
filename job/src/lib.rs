use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use pipeline::{
    Args as PlArgs, Command as PlCommand, ExecutionMode as PlExecutionMode, Pipeline as Pl,
    Runner as PlRunner, Step as PlStep, TaskId, Tasks,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub runners: Option<Vec<Runner>>,
    pub commands: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Runner {
    pub name: String,
    pub args: Option<Args>,
}

impl From<PlRunner> for Runner {
    fn from(runner: PlRunner) -> Self {
        Runner {
            name: runner.name,
            args: runner.args.map(From::from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    description: String,
    execution_mode: ExecutionMode,
    run: Vec<Command>,
    run_before: Option<Command>,
    run_after: Option<Command>,
    run_after_success: Option<Command>,
    run_after_error: Option<Command>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Task {
        id: TaskId,
        args: Option<Args>,
        execution_mode: ExecutionMode,
        run: Vec<Command>,
        run_before: Option<Box<Command>>,
        run_after: Option<Box<Command>>,
        run_after_success: Option<Box<Command>>,
        run_after_error: Option<Box<Command>>,
    },
    Plugin {
        uri: String,
        cmd: String,
        args: Option<Args>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Args {
    Map(HashMap<String, String>),
    List(Vec<String>),
}

impl From<PlArgs> for Args {
    fn from(args: PlArgs) -> Self {
        match args {
            PlArgs::Map(map) => Self::Map(map),
            PlArgs::List(list) => Self::List(list),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecutionMode {
    SequenceStopOnError,
    SequenceRunAll,
    Parallel,
}

impl From<PlExecutionMode> for ExecutionMode {
    fn from(args: PlExecutionMode) -> Self {
        match args {
            PlExecutionMode::SequenceStopOnError => Self::SequenceStopOnError,
            PlExecutionMode::SequenceRunAll => Self::SequenceRunAll,
            PlExecutionMode::Parallel => Self::Parallel,
        }
    }
}

pub fn from_pipeline(pipeline: Pl) -> Job {
    let Pl {
        steps,
        tasks,
        runners,
    } = pipeline;
    Job {
        runners: runners.map(|runners| runners.into_iter().map(From::from).collect()),
        commands: steps
            .into_iter()
            .map(|step| expand_step(step, &tasks))
            .collect(),
    }
}

fn expand_step(step: PlStep, existing_tasks: &Tasks) -> Step {
    let task = step.task;
    Step {
        description: step.description,
        execution_mode: task.execution_mode.into(),
        run: expand_cmds(task.run, existing_tasks),
        run_before: expand_optional_cmd(task.run_before, existing_tasks),
        run_after: expand_optional_cmd(task.run_after, existing_tasks),
        run_after_error: expand_optional_cmd(task.run_after_error, existing_tasks),
        run_after_success: expand_optional_cmd(task.run_after_success, existing_tasks),
    }
}

fn expand_cmds(cmds: Vec<PlCommand>, tasks: &Tasks) -> Vec<Command> {
    cmds.into_iter().map(|cmd| expand_cmd(cmd, tasks)).collect()
}

fn expand_optional_cmd(cmd: Option<PlCommand>, tasks: &Tasks) -> Option<Command> {
    cmd.map(|cmd| expand_cmd(cmd, tasks))
}

fn expand_cmd(cmd: PlCommand, tasks: &Tasks) -> Command {
    match cmd {
        PlCommand::Plugin { uri, cmd, args } => Command::Plugin {
            uri,
            cmd,
            args: args.map(From::from).take(),
        },
        PlCommand::Task { id, args } => expand_task(id, args, tasks),
    }
}

fn expand_task(id: TaskId, args: Option<PlArgs>, tasks: &Tasks) -> Command {
    if let Some(task) = tasks.get(&id) {
        let task = task.clone();
        Command::Task {
            id,
            args: args.map(From::from),
            execution_mode: task.execution_mode.into(),
            run: expand_cmds(task.run, tasks),
            run_before: expand_optional_cmd(task.run_before, tasks).map(Box::new),
            run_after: expand_optional_cmd(task.run_after, tasks).map(Box::new),
            run_after_error: expand_optional_cmd(task.run_after_error, tasks).map(Box::new),
            run_after_success: expand_optional_cmd(task.run_after_success, tasks).map(Box::new),
        }
    } else {
        unreachable!("pipeline check should catch this");
    }
}

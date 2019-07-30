use crate::{
    error::{Error, Result},
    pipeline::{Command, Pipeline as UncheckedPipeline, Runner, Step, Task, TaskId, Tasks},
};
use std::collections::HashMap;

pub struct Pipeline {
    pub runners: Option<Vec<Runner>>,
    pub steps: Vec<Step>,
    pub tasks: HashMap<TaskId, Task>,
}

pub(crate) fn check(pipeline: UncheckedPipeline) -> Result<Pipeline> {
    for step in &pipeline.steps {
        check_step(step, &pipeline.tasks)?;
    }

    Ok(Pipeline {
        runners: pipeline.runners,
        steps: pipeline.steps,
        tasks: pipeline.tasks,
    })
}
fn check_step(step: &Step, existing_tasks: &Tasks) -> Result<()> {
    check_cmds(&step.task.run, existing_tasks, Vec::new())?;
    check_optional_cmd(&step.task.run_before, existing_tasks, Vec::new())?;
    check_optional_cmd(&step.task.run_after, existing_tasks, Vec::new())?;
    check_optional_cmd(&step.task.run_after_error, existing_tasks, Vec::new())?;
    check_optional_cmd(&step.task.run_after_success, existing_tasks, Vec::new())?;

    Ok(())
}

fn check_cmds(cmds: &Vec<Command>, tasks: &Tasks, found: Vec<TaskId>) -> Result<()> {
    cmds.iter()
        .map(|cmd| check_cmd(cmd, tasks, found.clone()))
        .collect()
}

fn check_optional_cmd(cmd: &Option<Command>, tasks: &Tasks, found: Vec<TaskId>) -> Result<()> {
    cmd.as_ref()
        .map(|cmd| check_cmd(cmd, tasks, found))
        .unwrap_or(Ok(()))
}

fn check_cmd(cmd: &Command, tasks: &Tasks, mut found: Vec<TaskId>) -> Result<()> {
    match cmd {
        Command::Plugin { .. } => Ok(()),
        Command::Task { id, .. } => {
            if !found.contains(&id) {
                found.push(id.clone());
                check_task(id, tasks, found)
            } else {
                Err(Error::TaskRecursion(found))
            }
        }
    }
}

fn check_task(id: &TaskId, tasks: &Tasks, found: Vec<TaskId>) -> Result<()> {
    tasks
        .get(id)
        .ok_or_else(|| Error::TaskNotFound(id.clone()))
        .and_then(|task| {
            check_cmds(&task.run, tasks, found.clone())?;
            check_optional_cmd(&task.run_before, tasks, found.clone())?;
            check_optional_cmd(&task.run_after, tasks, found.clone())?;
            check_optional_cmd(&task.run_after_error, tasks, found.clone())?;
            check_optional_cmd(&task.run_after_success, tasks, found.clone())?;
            Ok(())
        })
}

use super::{data, pipeline::*};

impl From<data::Pipeline> for Pipeline {
    fn from(pipeline: data::Pipeline) -> Self {
        Self {
            steps: pipeline.steps.into_iter().map(From::from).collect(),
            inline: pipeline
                .inline
                .into_iter()
                .map(|(id, unit)| (id, From::from(unit)))
                .collect(),
        }
    }
}

impl From<data::Step> for Step {
    fn from(step: data::Step) -> Self {
        Self {
            description: step.description,
            run: From::from((step.run, step.execution_mode)),
            run_before: step.run_before.map(From::from),
            run_after: step.run_after.map(From::from),
            run_after_error: step.run_after_error.map(From::from),
            run_after_success: step.run_after_success.map(From::from),
        }
    }
}

impl From<data::Inline> for Inline {
    fn from(unit: data::Inline) -> Self {
        Self {
            run: From::from((unit.run, unit.execution_mode)),
            run_before: unit.run_before.map(From::from),
            run_after: unit.run_after.map(From::from),
            run_after_error: unit.run_after_error.map(From::from),
            run_after_success: unit.run_after_success.map(From::from),
        }
    }
}

impl From<(Vec<data::Command>, data::ExecutionMode)> for Commands {
    fn from((cmds, mode): (Vec<data::Command>, data::ExecutionMode)) -> Self {
        let cmds = cmds.into_iter().map(From::from).collect();

        match mode {
            data::ExecutionMode::SequenceStopOnError => Self::SequenceStopOnError(cmds),
            data::ExecutionMode::SequenceRunAll => Self::SequenceRunAll(cmds),
            data::ExecutionMode::Parallel => Self::Parallel(cmds),
        }
    }
}

impl From<data::Command> for Command {
    fn from(cmd: data::Command) -> Self {
        let args = cmd
            .args
            .iter()
            .map(|(key, value)| (key.to_owned(), value.into()))
            .collect();

        Self { id: cmd.id.into(), uri: cmd.uri, args }
    }
}

impl From<data::CommandId> for CommandId {
    fn from(id: data::CommandId) -> Self {
        match id {
            data::CommandId::Uuid(id) => Self::Uuid(id),
            data::CommandId::Named(id) => Self::Named(id),
        }
    }
}

impl From<&data::Argument> for Argument {
    fn from(args: &data::Argument) -> Self {
        match args {
            data::Argument::Map(map) => Self::Map(map.to_owned()),
            data::Argument::List(list) => Self::List(list.to_owned()),
            data::Argument::String(string) => Self::String(string.to_owned()),
        }
    }
}

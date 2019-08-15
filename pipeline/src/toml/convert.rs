use super::*;
use crate::pipeline as p;

impl Into<p::Pipeline> for Pipeline {
    fn into(self) -> p::Pipeline {
        p::Pipeline {
            steps: self.steps.into_iter().map(Into::into).collect(),
            units: self
                .units
                .into_iter()
                .map(|(id, unit)| (id, unit.into()))
                .collect(),
        }
    }
}

impl Into<p::Unit> for Unit {
    fn into(self) -> p::Unit {
        p::Unit {
            description: self.description,
            commands: self.commands.into(),
            args: self
                .args
                .map(|args| args.into_iter().map(Into::into).collect()),
        }
    }
}

impl Into<p::ArgumentKey> for ArgumentKey {
    fn into(self) -> p::ArgumentKey {
        p::ArgumentKey { name: self.name }
    }
}

impl Into<p::Commands> for Commands {
    fn into(self) -> p::Commands {
        match self {
            Commands::Single(cmd) => p::Commands::Single(cmd.into()),
            Commands::List(cmds) => p::Commands::Multiple {
                run_on_status: vec![p::Status::Success],
                mode: p::ExecutionMode::SequenceStopOnError,
                commands: cmds.into_iter().map(Into::into).collect(),
            },
            Commands::ConfiguredList {
                commands,
                mode,
                run_on_status,
            } => p::Commands::Multiple {
                mode: mode.into(),
                commands: commands.into_iter().map(Into::into).collect(),
                run_on_status: run_on_status.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl Into<p::ExecutionMode> for ExecutionMode {
    fn into(self) -> p::ExecutionMode {
        match self {
            Self::SequenceStopOnError => p::ExecutionMode::SequenceStopOnError,
            Self::SequenceRunAll => p::ExecutionMode::SequenceRunAll,
            Self::Parallel => p::ExecutionMode::Parallel,
        }
    }
}

impl Into<p::Status> for Status {
    fn into(self) -> p::Status {
        match self {
            Self::Success => p::Status::Success,
            Self::Error => p::Status::Error,
            Self::Abort => p::Status::Abort,
        }
    }
}

impl Into<p::Command> for Command {
    fn into(self) -> p::Command {
        match self {
            Self::Unit { id, name, args } => p::Command::Unit {
                id,
                name,
                args: args.map(Into::into),
            },
            Self::Wasm {
                id,
                uri,
                command,
                args,
            } => p::Command::Wasm {
                id,
                uri,
                command,
                args: args.map(Into::into),
            },
            Self::Oci {
                id,
                repository,
                image,
                command,
                raw_command,
                args,
                force_rebuild,
            } => p::Command::Oci {
                id,
                repository,
                raw_command,
                image,
                command,
                args: args.map(Into::into),
                force_rebuild,
            },
        }
    }
}

impl Into<p::Arguments> for Arguments {
    fn into(self) -> p::Arguments {
        match self {
            Self::Map(map) => p::Arguments::Map(map.to_owned()),
            Self::List(list) => p::Arguments::List(list.to_owned()),
            Self::String(string) => p::Arguments::String(string.to_owned()),
        }
    }
}

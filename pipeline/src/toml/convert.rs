use std::convert::TryInto;
use std::ops::Deref;
use super::{*, error::Error};
use crate::pipeline as p;

type Result<T> = std::result::Result<T, Error>;

impl TryInto<p::Pipeline> for Pipeline {
    type Error = Error;

    fn try_into(self) -> Result<p::Pipeline> {
        Ok(p::Pipeline {
            steps: self.steps.into_iter().map(TryInto::try_into).collect::<Result<Vec<p::Unit>>>()?,
            units: self
                .units
                .into_iter()
                .map(|(id, unit)| match unit.try_into() {
                    Ok(unit) => Ok((id, unit)),
                    Err(err) => Err(err),
                })
                .collect::<Result<p::Units>>()?,
        })
    }
}

impl TryInto<p::Unit> for Unit {
    type Error = Error;

    fn try_into(self) -> Result<p::Unit> {
        Ok(p::Unit {
            description: self.description,
            commands: self.commands.try_into()?,
            args: self
                .args
                .map(|args| args.into_iter().map(Into::into).collect()),
        })
    }
}

impl Into<p::ArgumentKey> for ArgumentKey {
    fn into(self) -> p::ArgumentKey {
        p::ArgumentKey { name: self.name }
    }
}

impl TryInto<p::Commands> for Commands {
    type Error = Error;
    fn try_into(self) -> Result<p::Commands> {
        Ok(match self {
            Commands::Single(cmd) => p::Commands::Single(cmd.try_into()?),
            Commands::List(cmds) => p::Commands::Multiple {
                run_on_status: vec![p::Status::Success],
                mode: p::ExecutionMode::SequenceStopOnError,
                commands: cmds.into_iter().map(TryInto::try_into).collect::<Result<Vec<p::Commands>>>()?,
            },
            Commands::ConfiguredList {
                commands,
                mode,
                run_on_status,
            } => p::Commands::Multiple {
                mode: mode.into(),
                commands: commands.into_iter().map(TryInto::try_into).collect::<Result<Vec<p::Commands>>>()?,
                run_on_status: run_on_status.into_iter().map(Into::into).collect(),
            },
        })
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

impl TryInto<p::Command> for Command {
    type Error = Error;

    fn try_into(self) -> Result<p::Command> {
        Ok(match self {
            Self::Unit { id, args } => p::Command::Unit {
                instance_id: p::InstanceId::new_v4(),
                id,
                args: args.map(TryInto::try_into).transpose()?,
            },
            Self::If { condition, then, othewise } => p::Command::If {
                condition: Box::new(condition.deref().to_owned().into()?),
                then: Box::new(then.deref().to_owned().into()?),
                //othewise: othewise.map(TryInto::try_into).transpose()?,
                othewise: None,
            },
            Self::Wasm { uri, command, commands } => p::Command::Wasm {
                instance_id: p::InstanceId::new_v4(),
                uri,
                commands: map_commands(command, commands)?,
            },
            Self::Oci { repository, image, command, commands, } => p::Command::Oci {
                instance_id: p::InstanceId::new_v4(),
                repository,
                image,
                commands: map_commands(command, commands)?,
            },
        })
    }
}

fn map_commands(single: Option<ExternalCommand>, multiple: Option<Vec<ExternalCommand>>) -> Result<Vec<p::ExternalCommand>> {
    let commands = match (single, multiple) {
        (Some(_), Some(_)) => return Err(Error::CommandMix),
        (None, None) => return Err(Error::NoCommandFound),
        (Some(single), _) => vec![single],
        (_, Some(multiple)) => multiple,
    };

    Ok(commands.into_iter().map(|cmd| cmd.into()).collect())
}

impl Into<p::ExternalCommand> for ExternalCommand {
    fn into(self) -> p::ExternalCommand {
        p::ExternalCommand {
            command: self.command,
            args: self.args.map(Into::into),
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

impl TryInto<HashMap<String, String>> for Arguments {
    type Error = Error;

    fn try_into(self) -> Result<HashMap<String, String>> {
        match self {
            Arguments::String(_) => Err(Error::InvalidArgumentsType("argument string", "argument map")),
            Arguments::List(_) => Err(Error::InvalidArgumentsType("argument list", "argument map")),
            Arguments::Map(args) => Ok(args),
        }
    }
}

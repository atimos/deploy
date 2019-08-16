use std::collections::HashMap;
use super::{
    error::Error,
    pipeline::{ArgumentKey, Command, Commands, Pipeline, Unit, Units},
};

pub fn check(pipeline: Pipeline) -> Result<Pipeline, Error> {
    check_steps(&pipeline.steps, &pipeline.units)?;
    check_units(&pipeline.units)?;
    Ok(pipeline)
}

fn check_steps(steps: &Vec<Unit>, units: &Units) -> Result<(), Error> {
    steps
        .iter()
        .map(|step| check_cmds(&step.commands, units, Vec::new()))
        .collect()
}

fn check_units(units: &Units) -> Result<(), Error> {
    units
        .iter()
        .map(|(_, unit)| check_cmds(&unit.commands, units, Vec::new()))
        .collect()
}

fn check_cmds(cmd: &Commands, units: &Units, used: Vec<String>) -> Result<(), Error> {
    match cmd {
        Commands::Single(cmd) => check_cmd(cmd, units, used),
        Commands::Multiple { commands, .. } => commands
            .iter()
            .map(|cmds| check_cmds(cmds, units, used.clone()))
            .collect(),
    }
}

fn check_cmd(cmd: &Command, units: &Units, mut found: Vec<String>) -> Result<(), Error> {
    match cmd {
        Command::Oci { .. } => Ok(()),
        Command::Wasm { .. } => Ok(()),
        Command::Unit { id, args, .. } => {
            if found.contains(id) {
                found.push(id.to_owned());
                return Err(Error::UnitRecursion(found));
            }
            if let Some(unit) = units.get(id) {
                found.push(id.to_owned());
                check_args(args, &unit.args)?;
                check_cmds(&unit.commands, units, found)
            } else {
                Err(Error::UnitNotFound(id.to_owned()))
            }
        }
    }
}

fn check_args(
    cmd_args: &Option<HashMap<String, String>>,
    unit_args: &Option<Vec<ArgumentKey>>,
) -> Result<(), Error> {
    if let Some(cmd_args) = cmd_args {
        if let Some(unit_args) = unit_args {
            for ArgumentKey { name } in unit_args {
                if !cmd_args.contains_key(name) {
                    return Err(Error::ArgumentMissing(name.to_owned()));
                }
            }
            'args: for arg_name in cmd_args.keys() {
                for ArgumentKey { name } in unit_args {
                    if name == arg_name {
                        continue 'args;
                    }
                }
                return Err(Error::UnexpectedArgument(arg_name.to_owned()));
            }
            Ok(())
        } else {
            Err(Error::UnexpectedArguments)
        }
    } else {
        if unit_args.is_none() {
            Ok(())
        } else {
            Err(Error::ArgumentsMissing)
        }
    }
}

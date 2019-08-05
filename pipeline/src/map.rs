use std::{collections::HashMap, convert::TryFrom};

use super::{data, error::Error, pipeline::*, Url};

type InlineUnits = HashMap<String, data::Unit>;

impl TryFrom<data::Pipeline> for Pipeline {
    type Error = super::error::Error;

    fn try_from(pipeline: data::Pipeline) -> Result<Self, Self::Error> {
        let steps = pipeline.steps;
        let inline = pipeline.inline;

        Ok(Pipeline(
            steps
                .into_iter()
                .map(|step| Step::try_from((step, &inline)))
                .collect::<Result<Vec<Step>, Error>>()?,
        ))
    }
}

impl TryFrom<(data::Step, &InlineUnits)> for Step {
    type Error = Error;

    fn try_from((step, inlined_units): (data::Step, &InlineUnits)) -> Result<Self, Self::Error> {
        Ok(Step {
            description: step.description,
            unit: Unit::Inline {
                uri: None,
                args: HashMap::new(),
                run: map_list(&step.run, step.execution_mode, inlined_units, Vec::new())?,
                run_before: map_optional(&step.run_before, inlined_units, Vec::new())?,
                run_after: map_optional(&step.run_after, inlined_units, Vec::new())?,
                run_after_error: map_optional(&step.run_after_error, inlined_units, Vec::new())?,
                run_after_success: map_optional(
                    &step.run_after_success,
                    inlined_units,
                    Vec::new(),
                )?,
            },
        })
    }
}

impl TryFrom<(&data::Command, &InlineUnits, Vec<Url>)> for Unit {
    type Error = Error;

    fn try_from(
        (cmd, inlined_units, mut used): (&data::Command, &InlineUnits, Vec<Url>),
    ) -> Result<Self, Self::Error> {
        if used.contains(&cmd.uri) {
            used.push(cmd.uri.clone());
            return Err(Error::UnitRecursion(used));
        }

        used.push(cmd.uri.clone());

        let args = cmd
            .args
            .iter()
            .map(|(key, value)| (key.to_owned(), value.into()))
            .collect();

        if cmd.uri.scheme() == "inline" {
            let id = cmd
                .uri
                .domain()
                .ok_or_else(|| Error::DomainMissing(cmd.uri.clone()))?;
            let unit = inlined_units
                .get(id)
                .ok_or_else(|| Error::UnitNotFound(cmd.uri.clone()))?;

            Ok(Unit::Inline {
                uri: Some(cmd.uri.clone()),
                args: check_args(args, &unit.args)?,
                run: map_list(&unit.run, unit.execution_mode, inlined_units, used.clone())?,
                run_before: map_optional(&unit.run_before, inlined_units, used.clone())?,
                run_after: map_optional(&unit.run_after, inlined_units, used.clone())?,
                run_after_error: map_optional(&unit.run_after_error, inlined_units, used.clone())?,
                run_after_success: map_optional(
                    &unit.run_after_success,
                    inlined_units,
                    used,
                )?,
            })
        } else {
            Ok(Unit::Ref {
                uri: cmd.uri.to_owned(),
                args,
            })
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
fn map_list(
    cmds: &[data::Command],
    exec_mode: data::ExecutionMode,
    inlined_units: &InlineUnits,
    used: Vec<Url>,
) -> Result<Run, Error> {
    let cmds = cmds
        .iter()
        .map(|cmd| Unit::try_from((cmd, inlined_units, used.clone())))
        .collect::<Result<Vec<Unit>, Error>>()?;

    Ok(match exec_mode {
        data::ExecutionMode::SequenceStopOnError => Run::SequenceStopOnError(cmds),
        data::ExecutionMode::SequenceRunAll => Run::SequenceRunAll(cmds),
        data::ExecutionMode::Parallel => Run::Parallel(cmds),
    })
}

fn map_optional(
    cmd: &Option<data::Command>,
    inlined_units: &InlineUnits,
    used: Vec<Url>,
) -> Result<Option<Box<Unit>>, Error> {
    cmd.as_ref()
        .map(|cmd| Unit::try_from((cmd, inlined_units, used.clone())).map(Box::new))
        .transpose()
}

fn check_args(
    args: HashMap<String, Argument>,
    arg_keys: &[data::ArgumentKey],
) -> Result<HashMap<String, Argument>, Error> {
    for key in arg_keys {
        if !args.contains_key(&key.name) {
            return Err(Error::ArgumentMissing(key.name.to_string()));
        }
    }

    'args: for name in args.keys() {
        for key in arg_keys {
            if name == &key.name {
                continue 'args;
            }
        }
        return Err(Error::UnexpectedArgument(name.to_string()));
    }

    Ok(args)
}

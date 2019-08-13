use std::collections::HashMap;

use super::{data::*, error::Error, Url};

type InlineUnits = HashMap<String, Inline>;

pub fn check(pipeline: Pipeline) -> Result<Pipeline, Error> {
    pipeline
        .steps
        .iter()
        .map(check_step_commands(&pipeline.inline))
        .collect::<Result<(), Error>>()?;
    pipeline
        .inline
        .iter()
        .map(|(_, unit)| check_unit(unit, &pipeline.inline, Vec::new()))
        .collect::<Result<(), Error>>()?;
    Ok(pipeline)
}

fn check_step_commands<'a>(units: &'a InlineUnits) -> impl Fn(&'a Step) -> Result<(), Error> + 'a {
    move |step| {
        step.run
            .iter()
            .map(|cmd| check_cmd(cmd, units, Vec::new()))
            .collect::<Result<(), Error>>()?;
        step.run_before
            .as_ref()
            .map(|cmd| check_cmd(cmd, units, Vec::new()))
            .transpose()?;
        step.run_after
            .as_ref()
            .map(|cmd| check_cmd(cmd, units, Vec::new()))
            .transpose()?;
        step.run_after_error
            .as_ref()
            .map(|cmd| check_cmd(cmd, units, Vec::new()))
            .transpose()?;
        step.run_after_success
            .as_ref()
            .map(|cmd| check_cmd(cmd, units, Vec::new()))
            .transpose()?;
        Ok(())
    }
}

fn check_unit<'a>(unit: &Inline, units: &'a InlineUnits, used: Vec<Url>) -> Result<(), Error> {
    unit.run
        .iter()
        .map(|cmd| check_cmd(cmd, units, used.clone()))
        .collect::<Result<(), Error>>()?;
    unit.run_before
        .as_ref()
        .map(|cmd| check_cmd(cmd, units, used.clone()))
        .transpose()?;
    unit.run_after
        .as_ref()
        .map(|cmd| check_cmd(cmd, units, used.clone()))
        .transpose()?;
    unit.run_after_error
        .as_ref()
        .map(|cmd| check_cmd(cmd, units, used.clone()))
        .transpose()?;
    unit.run_after_success
        .as_ref()
        .map(|cmd| check_cmd(cmd, units, used.clone()))
        .transpose()?;
    Ok(())
}

fn check_cmd(cmd: &Command, units: &InlineUnits, mut used: Vec<Url>) -> Result<(), Error> {
    let domain = cmd
        .uri
        .domain()
        .ok_or_else(|| Error::DomainMissing(cmd.uri.to_owned()))?;

    if cmd.uri.scheme() != "inline" {
        return Ok(());
    }

    if !used.contains(&cmd.uri) {
        let unit = units
            .get(domain)
            .ok_or_else(|| Error::UnitNotFound(cmd.uri.clone()))?;

        used.push(cmd.uri.clone());

        check_unit(unit, units, used)?;

        Ok(())
    } else {
        used.push(cmd.uri.clone());
        Err(Error::UnitRecursion(used))
    }
}

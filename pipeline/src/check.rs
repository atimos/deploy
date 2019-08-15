use std::collections::HashMap;

use super::{
    error::Error,
    pipeline::{Pipeline, Unit},
};

type Units = HashMap<String, Unit>;

pub fn check(pipeline: Pipeline) -> Result<Pipeline, Error> {
    check_steps(&pipeline.steps, &pipeline.units)?;
    check_units(&pipeline.units)?;
    Ok(pipeline)
}

fn check_steps(steps: &Vec<Unit>, units: &Units) -> Result<(), Error> {
    Err(Error::UnitRecursion(Vec::new()))
}

fn check_units(units: &Units) -> Result<(), Error> {
    Err(Error::UnitRecursion(Vec::new()))
}
//
// fn check_step_commands<'a>(units: &'a InlineUnits) -> impl Fn(&'a Step) -> Result<(), Error> + 'a {
//     move |step| {
//         step.run
//             .iter()
//             .map(|cmd| check_cmd(cmd, units, Vec::new()))
//             .collect::<Result<(), Error>>()?;
//         step.run_before
//             .as_ref()
//             .map(|cmd| check_cmd(cmd, units, Vec::new()))
//             .transpose()?;
//         step.run_after
//             .as_ref()
//             .map(|cmd| check_cmd(cmd, units, Vec::new()))
//             .transpose()?;
//         step.run_after_error
//             .as_ref()
//             .map(|cmd| check_cmd(cmd, units, Vec::new()))
//             .transpose()?;
//         step.run_after_success
//             .as_ref()
//             .map(|cmd| check_cmd(cmd, units, Vec::new()))
//             .transpose()?;
//         Ok(())
//     }
// }
//
// fn check_unit<'a>(unit: &Inline, units: &'a InlineUnits, used: Vec<Url>) -> Result<(), Error> {
//     unit.run
//         .iter()
//         .map(|cmd| check_cmd(cmd, units, used.clone()))
//         .collect::<Result<(), Error>>()?;
//     unit.run_before
//         .as_ref()
//         .map(|cmd| check_cmd(cmd, units, used.clone()))
//         .transpose()?;
//     unit.run_after
//         .as_ref()
//         .map(|cmd| check_cmd(cmd, units, used.clone()))
//         .transpose()?;
//     unit.run_after_error
//         .as_ref()
//         .map(|cmd| check_cmd(cmd, units, used.clone()))
//         .transpose()?;
//     unit.run_after_success
//         .as_ref()
//         .map(|cmd| check_cmd(cmd, units, used.clone()))
//         .transpose()?;
//     Ok(())
// }
//
// fn check_cmd(cmd: &Command, units: &InlineUnits, mut used: Vec<Url>) -> Result<(), Error> {
//     let domain = cmd
//         .uri
//         .domain()
//         .ok_or_else(|| Error::DomainMissing(cmd.uri.to_owned()))?;
//
//     if cmd.uri.scheme() != "inline" {
//         return Ok(());
//     }
//
//     if !used.contains(&cmd.uri) {
//         let unit = units
//             .get(domain)
//             .ok_or_else(|| Error::UnitNotFound(cmd.uri.clone()))?;
//
//         used.push(cmd.uri.clone());
//
//         check_unit(unit, units, used)?;
//
//         Ok(())
//     } else {
//         used.push(cmd.uri.clone());
//         Err(Error::UnitRecursion(used))
//     }
// }

use super::Url;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pipeline(pub(crate) Vec<Step>);

impl Iterator for Pipeline {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() > 0 {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Step {
    pub description: String,
    pub unit: Unit,
}

#[derive(Debug)]
pub enum Unit {
    Ref {
        uri: Url,
        args: HashMap<String, Argument>,
    },
    Inline {
        uri: Option<Url>,
        args: HashMap<String, Argument>,
        run: Run,
        run_before: Option<Box<Unit>>,
        run_after: Option<Box<Unit>>,
        run_after_success: Option<Box<Unit>>,
        run_after_error: Option<Box<Unit>>,
    },
}

#[derive(Debug)]
pub enum Argument {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}

#[derive(Debug)]
pub enum Run {
    SequenceStopOnError(Vec<Unit>),
    SequenceRunAll(Vec<Unit>),
    Parallel(Vec<Unit>),
}

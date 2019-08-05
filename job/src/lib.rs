use pipeline::{
    Pipeline,
};

#[derive(Debug)]
pub struct Job {
    pub pipeline: Pipeline,
}

impl From<Pipeline> for Job {
    fn from(pipeline: Pipeline) -> Self {
        Job {
            pipeline
        }
    }
}

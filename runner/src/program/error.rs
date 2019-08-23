use handlebars::TemplateRenderError;

#[derive(Debug)]
pub enum Error {
    Template(TemplateRenderError),
    Wasm(super::wasm::Error),
    Oci(super::oci::Error),
}

impl From<TemplateRenderError> for Error {
    fn from(err: TemplateRenderError) -> Self {
        Self::Template(err)
    }
}

impl From<super::wasm::Error> for Error {
    fn from(err: super::wasm::Error) -> Self {
        Self::Wasm(err)
    }
}
impl From<super::oci::Error> for Error {
    fn from(err: super::oci::Error) -> Self {
        Self::Oci(err)
    }
}

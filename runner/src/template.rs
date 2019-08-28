use handlebars::{Handlebars, TemplateRenderError};
use pipeline::Arguments;

pub fn render(tpl: &str, args: &Arguments) -> Result<String, TemplateRenderError> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);
    hb.render_template(&tpl, &args)
}

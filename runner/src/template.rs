use serde_json::json;
use handlebars::{Handlebars, TemplateRenderError};
use pipeline::Arguments;

pub fn render(tpl: &str, args: &Option<Arguments>) -> Result<String, TemplateRenderError> {
    Ok(match args {
        Some(args) => {
            let args = match args {
                Arguments::String(args) => json!({ "args": args }),
                Arguments::List(args) => json!({ "args": args }),
                Arguments::Map(args) => json!({ "args": args }),
            };
            let mut hb = Handlebars::new();
            hb.set_strict_mode(true);
            hb.render_template(&tpl, &args)?
        }
        None => tpl.into()
    })
}

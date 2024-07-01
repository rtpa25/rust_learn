use handlebars::DirectorySourceOptions;

use crate::web::ctx;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Failed to render template {0}")]
    Render(#[from] handlebars::RenderError),
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>);

impl<'a> Renderer<'a> {
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();

        renderer
            .register_templates_directory(template_dir, DirectorySourceOptions::default())
            .expect("Failed to register base template");

        Self(renderer)
    }

    fn convert_to_value<S>(serilizable: &S) -> serde_json::Value
    where
        S: serde::Serialize + std::fmt::Debug,
    {
        serde_json::to_value(serilizable).expect("Failed to convert to value")
    }

    pub fn render<P>(&self, context: P, errors: &[&str]) -> String
    where
        P: ctx::PageContext + std::fmt::Debug + serde::Serialize,
    {
        let mut data = Self::convert_to_value(&context);
        if let Some(data) = data.as_object_mut() {
            data.insert("_errors".into(), errors.into()); // insert mannually to each page context
            data.insert("_title".into(), context.title().into());
            data.insert("_base".into(), context.parent().into());
        }

        self.do_render(context.template_path(), data)
    }

    fn do_render(&self, path: &str, ctx: serde_json::Value) -> String {
        self.0
            .render(path, &ctx)
            .expect("Failed to render template")
    }

    pub fn render_with_data<P, D>(&self, context: P, errors: &[&str], data: (&str, D)) -> String
    where
        P: ctx::PageContext + std::fmt::Debug + serde::Serialize,
        D: serde::Serialize + std::fmt::Debug,
    {
        use handlebars::to_json;
        let mut value = Self::convert_to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into()); // insert mannually to each page context
            value.insert("_title".into(), context.title().into());
            value.insert("_base".into(), context.parent().into());
            value.insert(data.0.into(), to_json(&data.1));
        }

        self.do_render(context.template_path(), value)
    }
}

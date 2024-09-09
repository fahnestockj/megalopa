mod spec_tests;
// A mustache compliant templating engine 🚀

trait render {
    fn render(
        &self,
        template_key: String,
        context: std::collections::HashMap<&str, CtxValue>,
    ) -> String;
}
struct TemplateEngine {
    templates: std::collections::HashMap<String, String>,
}

impl render for TemplateEngine {
    fn render(
        &self,
        template_key: String,
        context: std::collections::HashMap<&str, CtxValue>,
    ) -> String {
        todo!()
    }
}

trait oneoff_render {
    fn oneoff_render(&self, context: std::collections::HashMap<&str, CtxValue>) -> String;
}

impl oneoff_render for TemplateEngine {
    fn oneoff_render(&self, context: std::collections::HashMap<&str, CtxValue>) -> String {
        todo!()
    }
}

pub fn create_engine(templates_glob: &str) -> TemplateEngine {
    // read in templates
    todo!();
}

pub fn create_oneoff_engine(template_str: &str) -> TemplateEngine {
    todo!();
}
pub enum CtxValue {
  String(String),
  Boolean(bool),
  Number(i8),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: &'static str,
    pub args: Vec<(&'static str, fluent::FluentValue<'static>)>,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

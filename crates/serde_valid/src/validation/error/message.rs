use super::{Format, FormatDefault};

#[derive(Debug, Clone, PartialEq)]
pub struct Message<E> {
    error: E,
    format: Format<E>,
}

impl<E> Message<E> {
    pub fn new(error: E, format: Format<E>) -> Self {
        Self { error, format }
    }

    #[cfg(feature = "fluent")]
    pub fn fluent_message(&self) -> Option<&crate::features::fluent::Message> {
        match self.format {
            Format::Fluent(ref message) => Some(message),
            _ => None,
        }
    }
}

impl<E> FormatDefault for Message<E>
where
    E: FormatDefault,
{
    fn format_default(&self) -> String {
        match &self.format {
            Format::Default => self.error.format_default(),
            Format::Message(ref message) => message.to_string(),
            Format::MessageFn(ref format_fn) => format_fn(&self.error),
            #[cfg(feature = "fluent")]
            Format::Fluent(message) => format!("{message}"),
        }
    }
}

impl<E> std::fmt::Display for Message<E>
where
    E: FormatDefault,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_default())
    }
}

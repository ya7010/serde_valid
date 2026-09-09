#[derive(Debug, Default)]
pub enum Format<E> {
    #[default]
    Default,
    Message(String),
    MessageFn(fn(&E) -> String),
    #[cfg(feature = "fluent")]
    Fluent(crate::fluent::Message),
}

impl<E> Clone for Format<E> {
    fn clone(&self) -> Self {
        match self {
            Self::Default => Self::Default,
            Self::Message(message) => Self::Message(message.clone()),
            Self::MessageFn(format_fn) => Self::MessageFn(*format_fn),
            #[cfg(feature = "fluent")]
            Self::Fluent(message) => Self::Fluent(message.clone()),
        }
    }
}

impl<E> Format<E> {
    pub fn into_message(self, error: E) -> crate::validation::error::Message<E> {
        crate::validation::error::Message::new(error, self)
    }
}

pub trait FormatDefault {
    fn format_default(&self) -> String;
}

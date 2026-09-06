mod array_erros;
mod errors;
mod format;
mod into_error;
mod message;
mod object_errors;

use std::borrow::Cow;

#[allow(deprecated)]
pub use crate::error::EnumerateError;
pub use crate::error::{
    EnumError, ExclusiveMaximumError, ExclusiveMinimumError, MaxItemsError, MaxLengthError,
    MaxPropertiesError, MaximumError, MinItemsError, MinLengthError, MinPropertiesError,
    MinimumError, MultipleOfError, PatternError, UniqueItemsError,
};
pub use array_erros::ArrayErrors;
pub use errors::Errors;
pub use format::{Format, FormatDefault};
use indexmap::IndexMap;
pub use into_error::IntoError;
pub use message::Message;
pub use object_errors::ObjectErrors;

#[derive(Debug, Clone, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Minimum(Message<MinimumError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Maximum(Message<MaximumError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ExclusiveMinimum(Message<ExclusiveMinimumError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ExclusiveMaximum(Message<ExclusiveMaximumError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MultipleOf(Message<MultipleOfError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinLength(Message<MinLengthError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxLength(Message<MaxLengthError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Pattern(Message<PatternError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinItems(Message<MinItemsError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxItems(Message<MaxItemsError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    UniqueItems(Message<UniqueItemsError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinProperties(Message<MinPropertiesError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxProperties(Message<MaxPropertiesError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Enum(Message<EnumError>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Custom(String),

    #[error(transparent)]
    Items(ArrayErrors),

    #[error(transparent)]
    Properties(ObjectErrors),

    #[cfg(feature = "fluent")]
    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Fluent(crate::fluent::Message),
}

fn serialize_error_message<T, S>(message: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::Serializer,
{
    serializer.serialize_str(&message.to_string())
}

pub type VecErrors<E = crate::validation::Error> = Vec<E>;
pub type ItemErrorsMap<E> = IndexMap<usize, Errors<E>>;
pub type ItemVecErrorsMap<E> = IndexMap<usize, VecErrors<E>>;
pub type PropertyErrorsMap<E> = IndexMap<Cow<'static, str>, Errors<E>>;
pub type PropertyVecErrorsMap<E> = IndexMap<Cow<'static, str>, VecErrors<E>>;

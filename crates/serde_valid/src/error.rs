use itertools::Itertools;
use serde_valid_literal::Literal;

use crate::validation::error::FormatDefault;
use crate::validation::Number;

#[derive(Debug, thiserror::Error)]
pub enum Error<E>
where
    E: 'static + std::error::Error,
{
    #[error(transparent)]
    DeserializeError(#[from] E),

    #[error(transparent)]
    ValidationError(crate::validation::Errors<crate::validation::Error>),
}

impl<E> Error<E>
where
    E: 'static + std::error::Error,
{
    pub fn is_serde_error(&self) -> bool {
        match self {
            Self::DeserializeError(_) => true,
            Self::ValidationError(_) => false,
        }
    }

    pub fn as_serde_error(&self) -> Option<&E> {
        match self {
            Self::DeserializeError(error) => Some(error),
            Self::ValidationError(_) => None,
        }
    }

    pub fn is_validation_errors(&self) -> bool {
        match self {
            Self::DeserializeError(_) => false,
            Self::ValidationError(_) => true,
        }
    }

    pub fn as_validation_errors(&self) -> Option<&crate::validation::Errors> {
        match self {
            Self::DeserializeError(_) => None,
            Self::ValidationError(error) => Some(error),
        }
    }
}

macro_rules! struct_error_params {
    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $Error:ident {
            pub $limit:ident: Vec<$type:ty>,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $Error {
            pub $limit: Vec<$type>,
        }

        impl $Error {
            pub fn new<T>($limit: &[T]) -> Self
            where
                T: Into<$type> + std::fmt::Debug + Clone,
            {
                Self {
                    $limit: (*$limit).iter().map(|x| x.clone().into()).collect(),
                }
            }
        }

        impl FormatDefault for $Error {
            #[inline]
            fn format_default(&self) -> String {
                format!(
                    $default_message,
                    self.$limit.iter().map(|v| format!("{}", v)).join(", ")
                )
            }
        }
    };

    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $Error:ident {
            pub $limit:ident: $type:ty,
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $Error {
            pub $limit: $type,
        }

        impl $Error {
            pub fn new<N: Into<$type>>($limit: N) -> Self {
                Self {
                    $limit: $limit.into(),
                }
            }
        }

        impl FormatDefault for $Error {
            #[inline]
            fn format_default(&self) -> String {
                format!($default_message, self.$limit)
            }
        }
    };

    (
        #[derive(Debug, Clone)]
        #[default_message=$default_message:literal]
        pub struct $Error:ident;
    ) => {
        #[derive(Debug, Clone)]
        pub struct $Error;

        impl FormatDefault for $Error {
            #[inline]
            fn format_default(&self) -> String {
                format!($default_message)
            }
        }
    };
}

// Number
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `>= {}`."]
    pub struct MinimumError {
        pub minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `<= {}`."]
    pub struct MaximumError {
        pub maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `> {}`."]
    pub struct ExclusiveMinimumError {
        pub exclusive_minimum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The number must be `< {}`."]
    pub struct ExclusiveMaximumError {
        pub exclusive_maximum: Number,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The value must be multiple of `{}`."]
    pub struct MultipleOfError {
        pub multiple_of: Number,
    }
);

// String
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the value must be `>= {}`."]
    pub struct MinLengthError {
        pub min_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the value must be `<= {}`."]
    pub struct MaxLengthError {
        pub max_length: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The value must match the pattern of \"{0}\"."]
    pub struct PatternError {
        pub pattern: String,
    }
);

// Array
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the items must be `<= {}`."]
    pub struct MaxItemsError {
        pub max_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The length of the items must be `>= {}`."]
    pub struct MinItemsError {
        pub min_items: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The items must be unique."]
    pub struct UniqueItemsError;
);

// Object
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The size of the properties must be `<= {}`."]
    pub struct MaxPropertiesError {
        pub max_properties: usize,
    }
);

struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The size of the properties must be `>= {}`."]
    pub struct MinPropertiesError {
        pub min_properties: usize,
    }
);

// Generic
struct_error_params!(
    #[derive(Debug, Clone)]
    #[default_message = "The value must be in [{:}]."]
    pub struct EnumError {
        pub enumerate: Vec<Literal>,
    }
);

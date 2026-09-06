use fluent::{bundle::FluentBundle, FluentArgs, FluentError, FluentResource};

use crate::validation::error::{
    ArrayErrors, Errors, FormatDefault, ItemErrorsMap, ObjectErrors, PropertyErrorsMap, VecErrors,
};

use super::LocalizedError;

pub trait TryLocalize {
    type Target;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind;
}

impl TryLocalize for Errors<crate::validation::Error> {
    type Target = Errors<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        match self {
            Errors::Array(array) => Ok(Errors::Array(array.try_localize(bundle)?)),
            Errors::Object(object) => Ok(Errors::Object(object.try_localize(bundle)?)),
            Errors::NewType(newtype) => Ok(Errors::NewType(newtype.try_localize(bundle)?)),
        }
    }
}

impl TryLocalize for ArrayErrors<crate::validation::Error> {
    type Target = ArrayErrors<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        match (
            self.errors.try_localize(bundle),
            self.items.try_localize(bundle),
        ) {
            (Ok(errors), Ok(items)) => Ok(ArrayErrors { errors, items }),
            (Err(errors), Ok(_)) => Err(errors)?,
            (Ok(_), Err(items)) => Err(items)?,
            (Err(errors), Err(items)) => Err(errors.into_iter().chain(items).collect()),
        }
    }
}

impl TryLocalize for ObjectErrors<crate::validation::Error> {
    type Target = ObjectErrors<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        match (
            self.errors.try_localize(bundle),
            self.properties.try_localize(bundle),
        ) {
            (Ok(errors), Ok(properties)) => Ok(ObjectErrors { errors, properties }),
            (Err(errors), Ok(_)) => Err(errors)?,
            (Ok(_), Err(properties)) => Err(properties)?,
            (Err(errors), Err(properties)) => Err(errors.into_iter().chain(properties).collect()),
        }
    }
}

impl TryLocalize for VecErrors<crate::validation::Error> {
    type Target = VecErrors<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        self.iter()
            .map(|error| error.try_localize(bundle))
            .collect()
    }
}

impl TryLocalize for ItemErrorsMap<crate::validation::Error> {
    type Target = ItemErrorsMap<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        let mut errors = vec![];
        let target = self
            .iter()
            .filter_map(|(index, error)| {
                error
                    .try_localize(bundle)
                    .map(|error| Some((*index, error)))
                    .unwrap_or_else(|err| {
                        errors.extend(err);
                        None
                    })
            })
            .collect::<Self::Target>();

        if errors.is_empty() {
            Ok(target)
        } else {
            Err(errors)
        }
    }
}

impl TryLocalize for PropertyErrorsMap<crate::validation::Error> {
    type Target = PropertyErrorsMap<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        let mut errors = vec![];
        let target = self
            .iter()
            .filter_map(|(properties, error)| {
                error
                    .try_localize(bundle)
                    .map(|error| Some((properties.clone(), error)))
                    .unwrap_or_else(|err| {
                        errors.extend(err);
                        None
                    })
            })
            .collect::<Self::Target>();

        if errors.is_empty() {
            Ok(target)
        } else {
            Err(errors)
        }
    }
}

impl TryLocalize for crate::validation::Error {
    type Target = LocalizedError;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        match self {
            Self::Minimum(message) => message.try_localize(bundle),
            Self::Maximum(message) => message.try_localize(bundle),
            Self::ExclusiveMinimum(message) => message.try_localize(bundle),
            Self::ExclusiveMaximum(message) => message.try_localize(bundle),
            Self::MultipleOf(message) => message.try_localize(bundle),
            Self::MinLength(message) => message.try_localize(bundle),
            Self::MaxLength(message) => message.try_localize(bundle),
            Self::Pattern(message) => message.try_localize(bundle),
            Self::MinItems(message) => message.try_localize(bundle),
            Self::MaxItems(message) => message.try_localize(bundle),
            Self::UniqueItems(message) => message.try_localize(bundle),
            Self::MinProperties(message) => message.try_localize(bundle),
            Self::MaxProperties(message) => message.try_localize(bundle),
            Self::Enum(message) => message.try_localize(bundle),
            Self::Custom(message) => Ok(LocalizedError::String(message.to_string())),
            Self::Items(message) => Ok(LocalizedError::Items(message.try_localize(bundle)?)),
            Self::Properties(message) => {
                Ok(LocalizedError::Properties(message.try_localize(bundle)?))
            }
            Self::Fluent(message) => Ok(message
                .try_localize(bundle)?
                .unwrap_or_else(|| LocalizedError::String(message.id.to_string()))),
        }
    }
}

impl<E> TryLocalize for crate::validation::error::Message<E>
where
    E: FormatDefault,
{
    type Target = LocalizedError;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        if let Some(message) = self.fluent_message() {
            if let Some(localized) = message.try_localize(bundle)? {
                return Ok(localized);
            }
        }
        Ok(LocalizedError::String(self.format_default()))
    }
}

impl TryLocalize for crate::features::fluent::Message {
    type Target = Option<LocalizedError>;

    fn try_localize<M>(
        &self,
        bundle: &FluentBundle<FluentResource, M>,
    ) -> Result<Self::Target, Vec<FluentError>>
    where
        M: fluent::memoizer::MemoizerKind,
    {
        if let Some(msg) = bundle.get_message(self.id) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let args = FluentArgs::from_iter(self.args.to_owned());
                let value = bundle
                    .format_pattern(pattern, Some(&args), &mut errors)
                    .to_string();
                if errors.is_empty() {
                    return Ok(Some(LocalizedError::String(value)));
                } else {
                    return Err(errors);
                }
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use crate::fluent::Message;

    use super::*;
    use fluent::{bundle::FluentBundle, FluentResource, FluentValue};
    use serde_json::json;
    use unic_langid::LanguageIdentifier;

    #[test]
    fn try_localize_without_args() -> crate::tests::Result<()> {
        let ftl_string = "hello-world = Hello, world!".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        let error = crate::validation::Error::Fluent(Message {
            id: "hello-world",
            args: vec![],
        });

        assert_eq!(
            serde_json::to_value(error.try_localize(&bundle).expect("localization error."))?,
            json!("Hello, world!")
        );

        Ok(())
    }

    #[test]
    fn try_localize_with_args() -> crate::tests::Result<()> {
        let ftl_string = "intro = Welcome, { $name }.".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        let error = crate::validation::Error::Fluent(Message {
            id: "intro",
            args: vec![("name", FluentValue::from("John"))],
        });

        assert_eq!(
            serde_json::to_value(error.try_localize(&bundle).expect("localization error."))?,
            json!("Welcome, \u{2068}John\u{2069}.")
        );

        Ok(())
    }

    #[test]
    fn try_localize_from_validation_error() -> crate::tests::Result<()> {
        let ftl_string = "intro = Welcome, { $name }.".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        let error = crate::validation::Error::Maximum(
            crate::validation::error::Format::Fluent(Message {
                id: "intro",
                args: vec![("name", FluentValue::from("John"))],
            })
            .into_message(crate::MaximumError {
                maximum: serde_valid_literal::Number::I32(10),
            }),
        );

        assert_eq!(
            serde_json::to_value(error.try_localize(&bundle).expect("localization error."))?,
            json!("Welcome, \u{2068}John\u{2069}.")
        );

        Ok(())
    }
}

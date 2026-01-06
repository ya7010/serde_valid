use fluent::{bundle::FluentBundle, FluentResource};

use crate::validation::error::{
    ArrayErrors, Errors, FormatDefault, ItemErrorsMap, ObjectErrors, PropertyErrorsMap, VecErrors,
};

use super::{LocalizedError, TryLocalize};

pub trait Localize {
    type Target;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind;
}

impl Localize for Errors<crate::validation::Error> {
    type Target = Errors<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        match self {
            Errors::Array(array) => Errors::Array(array.localize(bundle)),
            Errors::Object(object) => Errors::Object(object.localize(bundle)),
            Errors::NewType(newtype) => Errors::NewType(newtype.localize(bundle)),
        }
    }
}

impl Localize for ArrayErrors<crate::validation::Error> {
    type Target = ArrayErrors<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        ArrayErrors {
            errors: self.errors.localize(bundle),
            items: self.items.localize(bundle),
        }
    }
}

impl Localize for ObjectErrors<crate::validation::Error> {
    type Target = ObjectErrors<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        ObjectErrors {
            errors: self.errors.localize(bundle),
            properties: self.properties.localize(bundle),
        }
    }
}

impl Localize for VecErrors<crate::validation::Error> {
    type Target = VecErrors<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        self.iter().map(|error| error.localize(bundle)).collect()
    }
}

impl Localize for ItemErrorsMap<crate::validation::Error> {
    type Target = ItemErrorsMap<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        self.iter()
            .map(|(index, error)| (*index, error.localize(bundle)))
            .collect()
    }
}

impl Localize for PropertyErrorsMap<crate::validation::Error> {
    type Target = PropertyErrorsMap<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        self.iter()
            .map(|(property, error)| ((*property).clone(), error.localize(bundle)))
            .collect()
    }
}

impl Localize for crate::validation::Error {
    type Target = LocalizedError;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        match self {
            Self::Minimum(message) => message.localize(bundle),
            Self::Maximum(message) => message.localize(bundle),
            Self::ExclusiveMinimum(message) => message.localize(bundle),
            Self::ExclusiveMaximum(message) => message.localize(bundle),
            Self::MultipleOf(message) => message.localize(bundle),
            Self::MinLength(message) => message.localize(bundle),
            Self::MaxLength(message) => message.localize(bundle),
            Self::Pattern(message) => message.localize(bundle),
            Self::MinItems(message) => message.localize(bundle),
            Self::MaxItems(message) => message.localize(bundle),
            Self::UniqueItems(message) => message.localize(bundle),
            Self::MinProperties(message) => message.localize(bundle),
            Self::MaxProperties(message) => message.localize(bundle),
            Self::Enum(message) => message.localize(bundle),
            Self::Custom(message) => LocalizedError::String(message.to_string()),
            Self::Items(message) => LocalizedError::Items(message.localize(bundle)),
            Self::Properties(message) => LocalizedError::Properties(message.localize(bundle)),
            Self::Fluent(message) => message.localize(bundle).unwrap_or_else(|| {
                LocalizedError::String(format!("Fluent id not found: \"{}\"", message.id))
            }),
        }
    }
}

impl<E> Localize for crate::validation::error::Message<E>
where
    E: FormatDefault,
{
    type Target = LocalizedError;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        self.try_localize(bundle)
            .unwrap_or_else(|_| LocalizedError::String(self.format_default()))
    }
}

impl Localize for crate::features::fluent::Message {
    type Target = Option<LocalizedError>;

    fn localize<M>(&self, bundle: &FluentBundle<FluentResource, M>) -> Self::Target
    where
        M: fluent::memoizer::MemoizerKind,
    {
        self.try_localize(bundle)
            .unwrap_or_else(|e: Vec<fluent::FluentError>| {
                Some(LocalizedError::String(format!("FluentErrors: {:?}", e)))
            })
    }
}

#[cfg(test)]
mod test {
    use crate::fluent::Message;

    use super::*;
    use fluent::{FluentBundle, FluentResource, FluentValue};
    use serde_json::json;
    use unic_langid::LanguageIdentifier;

    fn get_bundle() -> FluentBundle<FluentResource> {
        let ftl_string = ["hello-world = Hello, world!", "intro = Welcome, { $name }."]
            .join("\n")
            .to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        bundle
    }

    #[test]
    fn localize_without_args() -> crate::tests::Result<()> {
        let error = crate::validation::Error::Fluent(Message {
            id: "hello-world",
            args: vec![],
        });

        assert_eq!(
            serde_json::to_value(error.localize(&get_bundle()))?,
            json!("Hello, world!")
        );

        Ok(())
    }

    #[test]
    fn localize_fluetn_id_not_found() -> crate::tests::Result<()> {
        let ftl_string = "hello-world = Hello, world!".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        let error = crate::validation::Error::Fluent(Message {
            id: "hello",
            args: vec![],
        });

        assert_eq!(
            serde_json::to_value(error.localize(&bundle))?,
            json!("Fluent id not found: \"hello\"")
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
            serde_json::to_value(error.localize(&bundle))?,
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
            serde_json::to_value(error.localize(&bundle))?,
            json!("Welcome, \u{2068}John\u{2069}.")
        );

        Ok(())
    }
}

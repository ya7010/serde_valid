use std::hash::Hash;

use proc_macro2::Span;
use quote::{quote_spanned, ToTokens};

#[derive(Clone)]
pub struct WithWarnings<T> {
    pub data: T,
    pub warnings: Vec<Warning>,
}

impl<T> WithWarnings<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            warnings: vec![],
        }
    }

    pub fn new_with_warnings(data: T, warnings: Vec<Warning>) -> Self {
        Self { data, warnings }
    }

    pub fn from_iter(data: impl IntoIterator<Item = WithWarnings<T>>) -> WithWarnings<Vec<T>> {
        let mut warnings = vec![];
        let data = data
            .into_iter()
            .map(|WithWarnings { data, warnings: w }| {
                warnings.extend(w);
                data
            })
            .collect::<Vec<_>>();
        WithWarnings { data, warnings }
    }
}

impl<T> From<WithWarnings<T>> for WithWarnings<Vec<T>> {
    fn from(with_warnings: WithWarnings<T>) -> Self {
        WithWarnings {
            data: vec![with_warnings.data],
            warnings: with_warnings.warnings,
        }
    }
}

impl<T> From<T> for WithWarnings<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

#[derive(Clone)]
pub enum Warning {
    Deprecated {
        ident: syn::Ident,
        note: String,
        span: Span,
        lint_attrs: Vec<syn::Attribute>,
    },
}

impl Hash for Warning {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Deprecated { ident, note, .. } => {
                ident.hash(state);
                note.hash(state);
            }
        }
    }
}

impl std::cmp::PartialEq for Warning {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Deprecated {
                    ident: ident1,
                    note: note1,
                    ..
                },
                Self::Deprecated {
                    ident: ident2,
                    note: note2,
                    ..
                },
            ) => ident1 == ident2 && note1 == note2,
        }
    }
}

impl std::cmp::Eq for Warning {}

impl Warning {
    pub fn add_index(&self, index: usize) -> Self {
        match self {
            Self::Deprecated {
                ident,
                note,
                span,
                lint_attrs,
            } => Self::Deprecated {
                ident: syn::Ident::new(&format!("{}_{}", ident, index), ident.span()),
                note: note.clone(),
                span: *span,
                lint_attrs: lint_attrs.clone(),
            },
        }
    }

    pub fn with_lint_attrs(mut self, attrs: &[syn::Attribute]) -> Self {
        let lint_attrs = attrs
            .iter()
            .filter(|attr| {
                ["allow", "warn", "deny", "forbid"]
                    .iter()
                    .any(|name| attr.path().is_ident(name))
            })
            .cloned()
            .collect();

        match &mut self {
            Self::Deprecated {
                lint_attrs: warning_lint_attrs,
                ..
            } => *warning_lint_attrs = lint_attrs,
        }
        self
    }
}

impl ToTokens for Warning {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Deprecated {
                ident,
                note,
                span,
                lint_attrs,
            } => {
                let func_name = syn::Ident::new(
                    &format!("__{}", ident.to_string().to_lowercase()),
                    Span::mixed_site(),
                );

                quote_spanned!(*span =>
                    #(#lint_attrs)*
                    {
                        #[allow(dead_code)]
                        #[allow(clippy::let_unit_value)]
                        fn #func_name() {
                            #[deprecated(note = #note)]
                            #[allow(non_upper_case_globals)]
                            const _deprecated: () = ();
                            let _ = _deprecated;
                        }
                    }
                )
                .to_tokens(tokens)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_can_be_injected_with_the_call_site_lint_scope() {
        let warning = Warning::Deprecated {
            ident: syn::parse_quote!(legacy_rule),
            note: "use the replacement rule".to_owned(),
            span: Span::call_site(),
            lint_attrs: vec![],
        }
        .with_lint_attrs(&[syn::parse_quote!(#[allow(deprecated)])])
        .add_index(2);

        let tokens = warning.to_token_stream().to_string();

        assert!(tokens.contains("allow (deprecated)"));
        assert!(tokens.contains("deprecated"));
        assert!(tokens.contains("use the replacement rule"));
        assert!(tokens.contains("__legacy_rule_2"));
    }
}

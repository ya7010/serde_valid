use crate::attribute::{MetaListCustomMessage, MetaNameValueCustomMessage, MetaPathCustomMessage};
use crate::types::SingleIdentPath;
#[cfg(feature = "fluent")]
use crate::types::{CommaSeparatedNestedMetas, NestedMeta};
use crate::warning::WithWarnings;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

use super::lit::get_str;
pub type MessageFormat = TokenStream;

pub fn default_message_format() -> MessageFormat {
    quote!(::serde_valid::validation::error::Format::Default)
}

pub fn extract_custom_message_format(
    meta: &syn::Meta,
) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    let custom_message_path = match meta {
        syn::Meta::Path(path) => path,
        syn::Meta::List(list) => &list.path,
        syn::Meta::NameValue(name_value) => &name_value.path,
    };
    let custom_message_name = SingleIdentPath::new(custom_message_path)
        .map_err(|error| vec![error])?
        .ident()
        .to_string();

    match (
        MetaPathCustomMessage::from_str(&custom_message_name),
        MetaListCustomMessage::from_str(&custom_message_name),
        MetaNameValueCustomMessage::from_str(&custom_message_name),
        meta,
    ) {
        (Ok(_), _, _, syn::Meta::Path(_)) => {
            unreachable!()
        }
        (_, Ok(custom_message_type), _, syn::Meta::List(custom_message)) => Ok(
            extract_custom_message_format_from_meta_list(&custom_message_type, custom_message)?,
        ),
        (_, _, Ok(custom_message_type), syn::Meta::NameValue(custom_message)) => Ok(
            extract_custom_message_format_from_name_value(&custom_message_type, custom_message)?,
        ),
        (Ok(_), _, _, _) => Err(vec![crate::Error::meta_path_custom_message_need_value(
            custom_message_path,
            &custom_message_name,
        )]),
        (_, Ok(_), _, _) => Err(vec![crate::Error::meta_list_custom_message_need_value(
            custom_message_path,
            &custom_message_name,
        )]),
        (_, _, Ok(_), _) => Err(vec![
            crate::Error::meta_name_value_custom_message_need_value(
                custom_message_path,
                &custom_message_name,
            ),
        ]),
        _ => Err(vec![crate::Error::unknown_custom_message_type(
            custom_message_path,
            &custom_message_name,
        )]),
    }
}

#[allow(unused_variables)]
fn extract_custom_message_format_from_meta_list(
    custom_message_type: &MetaListCustomMessage,
    meta_list: &syn::MetaList,
) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    match custom_message_type {
        #[cfg(feature = "fluent")]
        message_type @ (MetaListCustomMessage::I18n | MetaListCustomMessage::Fluent) => {
            let path = &meta_list.path;
            let path_ident = SingleIdentPath::new(path)
                .map_err(|error| vec![error])?
                .ident();
            let message_fn_define = meta_list
                .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
                .map_err(|error| {
                    vec![crate::Error::custom_message_parse_error(path_ident, &error)]
                })?;
            get_fluent_message_from_meta(message_type, path, &message_fn_define)
        }
        #[allow(unreachable_patterns)]
        _ => unreachable!("This should not be called"),
    }
}

fn extract_custom_message_format_from_name_value(
    custom_message_type: &MetaNameValueCustomMessage,
    name_value: &syn::MetaNameValue,
) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    match custom_message_type {
        MetaNameValueCustomMessage::Message => get_message(&name_value.value),
        MetaNameValueCustomMessage::MessageFn => get_message_fn_from_meta_name_value(name_value),
        #[cfg(feature = "fluent")]
        MetaNameValueCustomMessage::MessageL10n => match &name_value.value {
            syn::Expr::Call(call) => get_fluent_message_from_call_expr(call),
            _ => Err(vec![crate::Error::l10n_need_fn_call(&name_value.value)]),
        },
    }
}

fn get_message_fn_from_meta_name_value(
    meta_name_value: &syn::MetaNameValue,
) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    let fn_define = match &meta_name_value.value {
        syn::Expr::Path(syn::ExprPath { path, .. }) => quote!(#path),
        syn::Expr::Call(call) => quote!(#call),
        syn::Expr::Closure(closure) => quote!(#closure),
        _ => Err(vec![
            crate::Error::message_fn_meta_name_value_needs_function_or_closure(meta_name_value),
        ])?,
    };

    Ok(WithWarnings::new(
        quote!(::serde_valid::validation::error::Format::MessageFn(#fn_define)),
    ))
}

fn get_message(expr: &syn::Expr) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    match expr {
        syn::Expr::Lit(lit) => {
            get_str(&lit.lit).map(|lit_str| quote!(::serde_valid::validation::error::Format::Message(#lit_str.to_string()))).map(WithWarnings::new)
        }
        _ => Err(vec![crate::Error::literal_only(expr)]),
    }
}

#[cfg(feature = "fluent")]
fn get_fluent_message_from_meta(
    message_type: &MetaListCustomMessage,
    path: &syn::Path,
    fn_define: &CommaSeparatedNestedMetas,
) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    use quote::ToTokens;

    use crate::types::CommaSeparatedTokenStreams;

    match fn_define.len() {
        0 => Err(vec![crate::Error::fluent_need_item(message_type, path)]),
        1 => {
            let id = get_fluent_id(&fn_define[0])
                .ok_or_else(|| vec![crate::Error::fluent_allow_key(message_type, &fn_define[0])])?;

            Ok(WithWarnings::new(quote!(
                ::serde_valid::validation::error::Format::Fluent(
                    ::serde_valid::fluent::Message{
                        id: #id,
                        args: vec![]
                    }
                )
            )))
        }
        _ => {
            let mut errors = vec![];
            let id = get_fluent_id(&fn_define[0])
                .ok_or_else(|| vec![crate::Error::fluent_allow_key(message_type, &fn_define[0])])?;

            let args = fn_define
                .iter()
                .skip(1)
                .filter_map(|arg| {
                    if let NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
                        let key = &name_value.path.to_token_stream().to_string();
                        let value = &name_value.value;
                        Some(quote!((#key, ::serde_valid::export::fluent::FluentValue::from(#value))))
                    } else {
                        errors.push(crate::Error::fluent_allow_args(message_type, arg));
                        None
                    }
                })
                .collect::<CommaSeparatedTokenStreams>();
            if errors.is_empty() {
                Ok(WithWarnings::new(quote!(
                    ::serde_valid::validation::error::Format::Fluent(
                        ::serde_valid::fluent::Message{
                            id: #id,
                            args: vec![#args]
                        }
                    )
                )))
            } else {
                Err(errors)
            }
        }
    }
}

#[cfg(feature = "fluent")]
fn get_fluent_message_from_call_expr(
    fn_define: &syn::ExprCall,
) -> Result<WithWarnings<MessageFormat>, crate::Errors> {
    use quote::ToTokens;

    if fn_define.func.to_token_stream().to_string() != "fluent" {
        Err(vec![crate::Error::l10n_fn_name_not_allow(&fn_define.func)])?
    };

    let mut fn_args = fn_define.args.iter();
    let fluent_id = match fn_args.next() {
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(fluent_id),
            ..
        })) => fluent_id,
        Some(expr) => Err(vec![crate::Error::fluent_id_must_be_str_lit(expr)])?,
        None => Err(vec![crate::Error::fluent_id_not_found(
            &fn_define.paren_token,
        )])?,
    };

    let mut errors = vec![];
    let fluent_args = TokenStream::from_iter(fn_args.filter_map(|arg| {
        if let syn::Expr::Assign(assign) = arg {
            let key = &assign.left.to_token_stream().to_string();
            let value = &assign.right;
            Some(quote!((#key, ::serde_valid::export::fluent::FluentValue::from(#value))))
        } else {
            errors.push(crate::Error::fluent_allow_arg(arg));
            None
        }
    }));

    if errors.is_empty() {
        Ok(WithWarnings::new(quote!(
            ::serde_valid::validation::error::Format::Fluent(
                ::serde_valid::fluent::Message{
                    id: #fluent_id,
                    args: vec![#fluent_args]
                }
            )
        )))
    } else {
        Err(errors)
    }
}

#[cfg(feature = "fluent")]
fn get_fluent_id(nested_meta: &NestedMeta) -> Option<&syn::LitStr> {
    match nested_meta {
        NestedMeta::Lit(syn::Lit::Str(id)) => Some(id),
        _ => None,
    }
}

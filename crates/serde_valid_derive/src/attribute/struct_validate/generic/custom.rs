use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use quote::quote;

pub fn extract_generic_struct_custom_validator_from_meta_name_value(
    meta_name_value: &syn::MetaNameValue,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match &meta_name_value.value {
        syn::Expr::Path(syn::ExprPath { path, .. }) => extract_struct_custom_from_meta_path(path),
        syn::Expr::Call(call) => extract_struct_custom_from_call(call),
        syn::Expr::Closure(closure) => extract_struct_custom_from_closure(closure),
        _ => Err(vec![
            crate::Error::validate_custom_meta_name_value_need_function_or_closure(meta_name_value),
        ]),
    }
}

fn extract_struct_custom_from_meta_path(meta_path: &syn::Path) -> Result<Validator, crate::Errors> {
    let rule_fn_name = &meta_path;
    let rule_vec_errors = crate::types::rule_vec_errors_ident();

    Ok(quote!(
        if let Err(__errors) = ::serde_valid::validation::custom::wrap_into_vec_errors(#rule_fn_name(self)) {
            #rule_vec_errors.extend(__errors);
        };
    ))
}

fn extract_struct_custom_from_call(call: &syn::ExprCall) -> Result<Validator, crate::Errors> {
    let rule_vec_errors = crate::types::rule_vec_errors_ident();
    Ok(quote!(
        if let Err(__errors) = ::serde_valid::validation::custom::wrap_call_validation(self, #call) {
            #rule_vec_errors.extend(__errors);
        };
    ))
}

fn extract_struct_custom_from_closure(
    closure: &syn::ExprClosure,
) -> Result<Validator, crate::Errors> {
    let rule_vec_errors = crate::types::rule_vec_errors_ident();
    Ok(quote!(
        if let Err(__errors) = ::serde_valid::validation::custom::wrap_closure_validation(self, #closure) {
            #rule_vec_errors.extend(__errors);
        };
    ))
}

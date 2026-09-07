use super::Field;
use quote::quote;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct NamedField<'a> {
    name: String,
    ident: syn::Ident,
    field: Cow<'a, syn::Field>,
}

impl<'a> NamedField<'a> {
    pub fn new(field: &'a syn::Field) -> Result<Self, crate::Error> {
        let Some(ident) = field.ident.as_ref() else {
            return Err(crate::Error::named_fields_struct_required(field));
        };
        let ident = ident.to_string();
        let name = ident.strip_prefix("r#").unwrap_or(&ident).to_owned();
        let generated_ident = crate::types::generated_ident(&format!("__serde_valid_{name}"));
        Ok(Self {
            name,
            ident: generated_ident,
            field: Cow::Borrowed(field),
        })
    }
}

impl Field for NamedField<'_> {
    fn name(&self) -> &String {
        &self.name
    }

    fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn key(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        quote!(::std::borrow::Cow::from(#name))
    }

    fn errors_variable(&self) -> proc_macro2::TokenStream {
        let property_vec_errors_map = crate::types::property_vec_errors_map_ident();
        quote!(#property_vec_errors_map)
    }

    fn getter_token(&self) -> proc_macro2::TokenStream {
        let ident = self.field.ident.as_ref().unwrap();
        quote!(#ident)
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        self.field.attrs.as_ref()
    }

    fn vis(&self) -> &syn::Visibility {
        &self.field.vis
    }

    fn ty(&self) -> &syn::Type {
        &self.field.ty
    }
}

use super::Field;
use quote::quote;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct NamedField<'a> {
    name: String,
    field: Cow<'a, syn::Field>,
}

impl<'a> NamedField<'a> {
    pub fn new(field: &'a syn::Field) -> Result<Self, crate::Error> {
        let Some(ident) = field.ident.as_ref() else {
            return Err(crate::Error::named_fields_struct_required(field));
        };
        Ok(Self {
            name: ident.to_string(),
            field: Cow::Borrowed(field),
        })
    }
}

impl Field for NamedField<'_> {
    fn name(&self) -> &String {
        &self.name
    }

    fn ident(&self) -> &syn::Ident {
        self.field.ident.as_ref().unwrap()
    }

    fn key(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        quote!(std::borrow::Cow::from(#name))
    }

    fn errors_variable(&self) -> proc_macro2::TokenStream {
        quote!(__property_vec_errors_map)
    }

    fn getter_token(&self) -> proc_macro2::TokenStream {
        let ident = self.ident();
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

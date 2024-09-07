use super::Field;
use proc_macro_error2::abort;
use quote::quote;
use std::borrow::Cow;
use std::convert::AsRef;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct UnnamedField<'a> {
    name: String,
    index: usize,
    ident: syn::Ident,
    field: Cow<'a, syn::Field>,
}

impl<'a> UnnamedField<'a> {
    pub fn new(index: usize, field: &'a syn::Field) -> Self {
        if field.ident.is_some() {
            abort!(field.span(), "struct must be unnamed fields struct.")
        }
        Self {
            name: index.to_string(),
            index,
            ident: syn::Ident::new(&format!("__{}", index), field.span()),
            field: Cow::Borrowed(field),
        }
    }
}

impl<'a> Field for UnnamedField<'a> {
    fn name(&self) -> &String {
        &self.name
    }

    fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn key(&self) -> proc_macro2::TokenStream {
        let index = self.index;
        quote!(#index)
    }

    fn errors_variable(&self) -> proc_macro2::TokenStream {
        quote!(__item_vec_errors_map)
    }

    fn getter_token(&self) -> proc_macro2::TokenStream {
        let index = syn::Index::from(self.index);
        quote!(#index)
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

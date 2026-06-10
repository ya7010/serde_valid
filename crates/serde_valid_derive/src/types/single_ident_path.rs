pub struct SingleIdentPath<'a>(&'a syn::Path);

impl<'a> SingleIdentPath<'a> {
    pub fn new(path: &'a syn::Path) -> Result<Self, crate::Error> {
        if path.get_ident().is_none() {
            return Err(crate::Error::path_must_be_single_ident(path));
        }
        Ok(Self(path))
    }

    pub fn ident(&self) -> &'a syn::Ident {
        self.0.get_ident().unwrap()
    }
}

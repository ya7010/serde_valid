pub trait IsMatch {
    fn is_match(&self, pattern: &regex::Regex) -> bool;
}

macro_rules! impl_for_str {
    ($ty:ty) => {
        impl IsMatch for $ty {
            fn is_match(&self, pattern: &regex::Regex) -> bool {
                pattern.is_match(self)
            }
        }
    };
}

impl_for_str!(str);
impl_for_str!(&str);
impl_for_str!(String);
impl_for_str!(std::borrow::Cow<'_, str>);

macro_rules! impl_for_os_str {
    ($ty:ty) => {
        impl IsMatch for $ty {
            fn is_match(&self, pattern: &regex::Regex) -> bool {
                pattern.is_match(&self.to_string_lossy())
            }
        }
    };
}

impl_for_os_str!(std::ffi::OsStr);
impl_for_os_str!(&std::ffi::OsStr);
impl_for_os_str!(std::ffi::OsString);
impl_for_os_str!(std::borrow::Cow<'_, std::ffi::OsStr>);

macro_rules! impl_for_path {
    ($ty:ty) => {
        impl IsMatch for $ty {
            fn is_match(&self, pattern: &regex::Regex) -> bool {
                self.as_os_str().is_match(pattern)
            }
        }
    };
}

impl_for_path!(std::path::Path);
impl_for_path!(&std::path::Path);
impl_for_path!(std::path::PathBuf);
impl_for_path!(std::borrow::Cow<'_, std::path::Path>);

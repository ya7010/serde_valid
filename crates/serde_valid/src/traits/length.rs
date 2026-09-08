use unicode_segmentation::UnicodeSegmentation;

pub trait Length {
    fn length(&self) -> usize;
}

macro_rules! impl_for_str {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.graphemes(true).count()
            }
        }
    };
}

impl_for_str!(str);
impl_for_str!(String);

macro_rules! impl_for_os_str {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.to_string_lossy().length()
            }
        }
    };
}

impl_for_os_str!(std::ffi::OsStr);
impl_for_os_str!(std::ffi::OsString);

macro_rules! impl_for_path {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.as_os_str().length()
            }
        }
    };
}

impl_for_path!(std::path::Path);
impl_for_path!(std::path::PathBuf);

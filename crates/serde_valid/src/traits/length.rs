use unicode_segmentation::UnicodeSegmentation;

pub trait Length {
    fn length(&self) -> usize;
}

impl<T> Length for &T
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        (*self).length()
    }
}

impl<T> Length for Box<T>
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<T> Length for std::rc::Rc<T>
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<T> Length for std::sync::Arc<T>
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<T> Length for std::borrow::Cow<'_, T>
where
    T: std::borrow::ToOwned + Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<P> Length for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: Length,
{
    fn length(&self) -> usize {
        self.as_ref().get_ref().length()
    }
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

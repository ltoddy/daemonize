use std::borrow::Cow;
use std::ffi::CStr;

/// An enum of all error kinds.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
    Io,
    Fork,
    DetachSession,
    DuplicateFileDescriptor,
}

enum Repr {
    Io(std::io::Error),
    Custom(ErrorKind, Cow<'static, str>),
}

pub struct Error {
    repr: Repr,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.repr {
            Repr::Io(ref e) => e.fmt(f),
            Repr::Custom(_, ref desc) => write!(f, "{}", desc),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error { repr: Repr::Io(e) }
    }
}

impl Error {
    pub fn custom<S: ToString>(kind: ErrorKind, desc: S) -> Error {
        Error {
            repr: Repr::Custom(kind, Cow::from(desc.to_string())),
        }
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Io(_) => ErrorKind::Io,
            Repr::Custom(kind, _) => kind,
        }
    }

    pub fn is_io_error(&self) -> bool {
        self.as_io_error().is_some()
    }

    pub fn as_io_error(&self) -> Option<&std::io::Error> {
        match self.repr {
            Repr::Io(ref e) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn errno() -> libc::c_int {
        std::io::Error::last_os_error().raw_os_error().unwrap()
    }

    pub(crate) fn strerror<'a>(n: libc::c_int) -> &'a str {
        unsafe { CStr::from_ptr(libc::strerror(n)).to_str().unwrap() }
    }
}

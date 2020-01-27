use failure::{Backtrace, Context, Fail};
use std::fmt;

/// The error kind for errors that get returned in the crate
#[derive(Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "failed to deserialize JSON")]
    JsonDeserializationFailed,
    #[fail(display = "failed to send request")]
    FailedToSendRequest,
    #[fail(display = "failed to read response")]
    FailedToReadResponse,
    #[fail(display = "failed to authenticate")]
    AuthenticationFailed,
    #[fail(display = "API call '{}' failed", _0)]
    ApiCallFailed(String),
}

impl Clone for ErrorKind {
    fn clone(&self) -> Self {
        use self::ErrorKind::*;
        match *self {
            JsonDeserializationFailed => JsonDeserializationFailed,
            FailedToSendRequest => FailedToSendRequest,
            FailedToReadResponse => FailedToReadResponse,
            AuthenticationFailed => AuthenticationFailed,
            ApiCallFailed(ref x) => ApiCallFailed(x.clone()),
        }
    }
}

/// The error type for errors that get returned in the lookup module
#[derive(Debug)]
pub struct Error {
    pub(crate) inner: Context<ErrorKind>,
}

impl Error {
    /// Get the kind of the error
    pub fn kind(&self) -> &ErrorKind { self.inner.get_context() }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Error {
            inner: Context::new(self.inner.get_context().clone()),
        }
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> { self.inner.cause() }

    fn backtrace(&self) -> Option<&Backtrace> { self.inner.backtrace() }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.inner, f) }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error { Error { inner } }
}

pub type Result<T> = ::std::result::Result<T, Error>;

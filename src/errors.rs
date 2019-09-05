/// # Fantom/libconsensus/errors
///
/// This file defines a set of errors which are used within the consensus traits.
use libcommon_rs::errors::Error as BaseError;
use libtransport::errors::Error as TransportError;

pub type Result<T> = std::result::Result<T, Error>;

/// Differentiates between multiple error types. Errors are outlined as follows:
/// Base: A base error, as defined in the libcommon crate.
/// AtMaxVecCapacity: Returned when a new element is added to a vector of full capacity.
/// Transport: An error caused by a transport protocol error.
#[derive(Debug)]
pub enum Error {
    Base(BaseError),
    // Error indicating Vec<T> is reached maximum capacity and would cause
    // panic while adding next element.
    AtMaxVecCapacity,
    Transport(TransportError),
}

/// Allow an error to be transformed into a BaseError.
impl From<BaseError> for Error {
    #[inline]
    fn from(b: BaseError) -> Error {
        Error::Base(b)
    }
}

/// Allow an error to be transformed into a TransportError
impl From<TransportError> for Error {
    #[inline]
    fn from(t: TransportError) -> Error {
        Error::Transport(t)
    }
}

/// What happens when a 'NONE' variant is unwrapped.
#[macro_export]
macro_rules! none_error {
    () => {
        libconsensus::errors::Error::Base(libcommon_rs::errors::Error::NoneError)
    };
}

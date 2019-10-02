/// # Fantom/libconsensus/errors
///
/// This file defines a set of errors which are used within the consensus traits.
use failure::Error as FailureError;
use libtransport::errors::Error as TransportError;

pub type Result<T> = std::result::Result<T, FailureError>;

/// Differentiates between multiple error types. Errors are outlined as follows:
/// Base: A base error, as defined in the libcommon crate.
/// AtMaxVecCapacity: Returned when a new element is added to a vector of full capacity.
/// Transport: An error caused by a transport protocol error.
#[derive(Debug, Fail)]
pub enum Error {
    // Error indicating Vec<T> is reached maximum capacity and would cause
    // panic while adding next element.
    #[fail(display = "Internal vector is at maximum capacity!")]
    AtMaxVecCapacity,
    #[fail(display = "Transport Error: {:?}", 0)]
    Transport(TransportError),
}

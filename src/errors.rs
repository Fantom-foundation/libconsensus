use libcommon_rs::errors::Error as BaseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Base(BaseError),
    // Error indicating Vec<T> is reached maximum capacity and would cause
    // panic while adding next element.
    AtMaxVecCapacity,
}

impl From<BaseError> for Error {
    #[inline]
    fn from(b: BaseError) -> Error {
        Error::Base(b)
    }
}

#[macro_export]
macro_rules! none_error {
    () => {
        libconsensus::errors::Error::Base(libcommon_rs::errors::Error::NoneError)
    };
}

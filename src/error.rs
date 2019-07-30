use std::{
    error::Error,
    fmt::{self, Display},
    sync::mpsc::RecvError,
};

#[derive(Debug)]
pub enum ConsensusError {
    EncodingError(String),
    SendError(String),
    ReceiveError(String),
    InvalidState(String),
    UnknownBlock(String),
    UnknownPeer(String),
    NoChainHead,
    BlockNotReady,
}

impl Display for ConsensusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ConsensusError::*;

        let error_message = match *self {
            EncodingError(ref s) => s,
            SendError(ref s) => s,
            ReceiveError(ref s) => s,
            InvalidState(ref s) => s,
            UnknownBlock(ref s) => s,
            UnknownPeer(ref s) => s,
            NoChainHead => "No chain head",
            BlockNotReady => "Block not ready to finalize",
        };

        write!(f, "({})", error_message)
    }
}

impl Error for ConsensusError {
    fn description(&self) -> &str {
        use self::ConsensusError::*;
        match *self {
            EncodingError(ref s) => s,
            SendError(ref s) => s,
            ReceiveError(ref s) => s,
            InvalidState(ref s) => s,
            UnknownBlock(ref s) => s,
            UnknownPeer(ref s) => s,
            NoChainHead => "No chain head",
            BlockNotReady => "Block not ready to finalize",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        use self::ConsensusError::*;
        match *self {
            EncodingError(_) => None,
            SendError(_) => None,
            ReceiveError(_) => None,
            InvalidState(_) => None,
            UnknownBlock(_) => None,
            UnknownPeer(_) => None,
            NoChainHead => None,
            BlockNotReady => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReceiveError {
    TimeoutError,
    ChannelError(RecvError),
    DisconnectedError,
}

impl Error for ReceiveError {
    fn description(&self) -> &str {
        match *self {
            ReceiveError::TimeoutError => "TimeoutError",
            ReceiveError::ChannelError(ref err) => err.description(),
            ReceiveError::DisconnectedError => "DisconnectedError",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ReceiveError::TimeoutError => None,
            ReceiveError::ChannelError(ref err) => Some(err),
            ReceiveError::DisconnectedError => None,
        }
    }
}

impl std::fmt::Display for ReceiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReceiveError::TimeoutError => write!(f, "TimeoutError"),
            ReceiveError::ChannelError(ref err) => write!(f, "ChannelError: {}", err.description()),
            ReceiveError::DisconnectedError => write!(f, "DisconnectedError"),
        }
    }
}

#[derive(Debug)]
pub enum SendError {
    DisconnectedError,
    TimeoutError,
    UnknownError,
}

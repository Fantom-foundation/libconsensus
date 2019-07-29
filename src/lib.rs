#[macro_use]
extern crate serde_derive;
use os_pipe::PipeWriter;
use std::sync::mpsc::Sender;

// Common type for Peer ID.
pub type PeerId = Vec<u8>;

// Base peer structure; common for various consenus algorithms
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BaseConsensusPeer {
    #[serde(rename = "PubKeyHex")]
    pub id: PeerId,
    #[serde(rename = "NetAddr")]
    pub net_addr: String,
}

// Internal transaction types
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    // add new peer into list of participants
    PeerAdd,
    // remove a peer from the list of participants
    // practically a cuicide as it need to be signed by leaving peer pk
    PeerRemove,
}

// Consensus trait for various distributed consensus algorithms implementations.
// Implementations must deliver finalised transactions in the following order:
// 1. push into all registered Rust channels
// 2. push into all registered os_pipes
// 3. call all registered callbacks
pub trait Consensus {
    // Consensus configuration type
    type Configuration;
    // Consensus data type; specify data type of events to be in consensus on order of.
    type Data: AsRef<u8>;

    // Create new Consensus instance
    fn new(cfg: Self::Configuration) -> Self;

    // Start up Consensus instance
    fn run(&mut self);

    // Shutdown Consensus instance
    fn shutdown(&mut self);

    // Send a transaction into Consensus
    // It returns True on successful send and False otherwise.
    fn send_transaction(&mut self, data: Self::Data) -> bool;

    // Register a callback function which is called when a transaction
    // is finalised in the consensus.
    // It returns True on successful registration and False otherwise.
    // Several callback function can be registered, they will be called in
    // the order of registration.
    fn register_callback(&mut self, callback: fn(data: Self::Data) -> bool) -> bool;
    // The callback function must return True when transaction is processed successfully and False otherwise.
    // The callback function will be called with the same transaction until
    // callback function returns True; a pause between  consecutive calls of the
    // callback function with the same block will be made for the value of milliseconds
    // set by callback_timeout() function of the Consensus trait below;
    // default value of the timeout is implementation defined.

    // Set timeout in milliseconds between consecutive calls of the callback
    // function with the same transaction.
    fn set_callback_timeout(&mut self, timeout: u64);

    // Register a sending-half of std::sync::mpsc::channel which is used to push
    // all finalised transaction to.
    // It returns True on successful registration and False otherwise
    // Several channels can be registered, they will be pushed in
    // the order of registration.
    fn register_channel(&mut self, sender: Sender<Self::Data>) -> bool;

    // Register a PipeWriter of os_pipe::pipe; which is used to push
    // all finalised transaction to.
    // It returns True on successful registration and False otherwise
    // Several pipes can be registered, they will be pushed in
    // the order of registration.
    fn register_os_pipe(&mut self, sender: PipeWriter) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::sync::mpsc::Sender;
use os_pipe::PipeWriter;

// Consensus trait for various distributed consensus algorithms implementations.
// Implementations must deliver finalised transactions in the following order:
// 1. push into all registered Rust channels
// 2. push into all registered os_pipes
// 3. call all registered callbacks
pub trait Consensus<B: AsRef<u8>> {
    // Consensus configuration type
    type Configuration;

    // Create new Consensus instance
    fn new(cfg: Self::Configuration) -> Self;

    // Start up Consensus instance
    fn run();

    // Shutdown Consensus instance
    fn shutdown();

    // Send a transaction into Consensus
    fn send_transaction (&mut self, data: B);

    // Register a callback function which is called when a transaction
    // is finalised in the consensus.
    // It returns True on successful registration and False otherwise
    // Several callback function can be registered, they will be called in
    // the order of registration.
    fn register_callback (&mut self, callback: fn(data: B)->bool) -> bool;
    // The callback function must return True when transaction is processed successfully and False otherwise.
    // The callback function will be called with the same transaction until
    // callback function returns True; a pause between  consecutive calls of the
    // callback function with the same block will be made for the value of milliseconds
    // set by callback_timeout() function of the Consensus trait below;
    // default value of the timeout is implementation defined.

    // Set timeout in milliseconds between consecutive calls of the callback
    // function with the same transaction.
    fn set_callback_timeout (&mut self, timeout: u64);

    // Register a sending-half of std::sync::mpsc::channel which is used to push
    // all finalised transaction to.
    // It returns True on successful registration and False otherwise
    // Several channels can be registered, they will be pushed in
    // the order of registration.
     fn register_channel (&mut self, sender: Sender<B>) -> bool;

    // Register a PipeWriter of os_pipe::pipe; which is used to push
    // all finalised transaction to.
    // It returns True on successful registration and False otherwise
    // Several pipes can be registered, they will be pushed in
    // the order of registration.
     fn register_os_pipe (&mut self, sender: PipeWriter) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

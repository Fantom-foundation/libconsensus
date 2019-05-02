
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

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

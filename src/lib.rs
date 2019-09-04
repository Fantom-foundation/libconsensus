/// # Fantom/libconsensus
///
/// This crate defines a set of commonly used traits which can be used for various consensus
/// implementations. The crate defines two traits: ConsensusConfiguration and Consensus. The crate
/// also defines two base structs which are commonly used between multiple consensus algorithms.
///
/// For an example of an implementation of the traits, refer to the libconsensus-dag repository:
/// https://github.com/Fantom-foundation/libconsensus-dag.
extern crate serde_derive;
use crate::errors::Result;
use futures::stream::Stream;
use libcommon_rs::peer::{Peer, PeerId};
use serde::{Deserialize, Serialize};

// Base peer structure; common for various consenus algorithms
/// A base structure for consensus peers which can be commonly used for multiple consensus algorithms.
/// The struct take in an Id type and a net address of the peer.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BaseConsensusPeer<P> {
    #[serde(rename = "PubKeyHex")]
    pub id: P,
    #[serde(rename = "NetAddr")]
    pub net_addr: String,
}

/// An enum for differentiating between transaction types which can occur in the network.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    // add new peer into list of participants
    PeerAdd,
    // remove a peer from the list of participants
    // practically a suicide as it need to be signed by leaving peer pk
    PeerRemove,
}

/// A simple trait for defining how a consensus algorithm is configured.
/// Has one implemented function: new(), which constructs a new Configuration type.
pub trait ConsensusConfiguration<Data> {
    // creates new consensus configuration
    fn new() -> Self;
}

/// The consensus trait - defines common functionality for multiple distributed consensus
/// algorithms. The requires a 'Data' type (D) which will be transferred between peers. The Data
/// type must also implement the 'Stream' trait so it can work asynchronously. Finally, all
/// implementations must deliver finalised transactions as the output of the async future.
pub trait Consensus<'de, D>: Stream<Item = D> + Drop
where
    D: Serialize + Deserialize<'de>,
{
    /// Define the configuration type required by the consensus algorithm.
    type Configuration: ConsensusConfiguration<D>;

    /// Creates a new instance of the consensus algorithm.
    fn new(cfg: Self::Configuration) -> Result<Self>
    where
        Self: Sized;

    /// Close the consensus instance.
    fn shutdown(&mut self) -> Result<()>;

    /// Send a transaction (type D). This function returns the result of the transaction (true or
    /// false).
    fn send_transaction(&mut self, data: D) -> Result<()>;
}

/// A an implementation of the Peer trait (found in libcommon repository) for the BaseConsensusPeer
/// struct (defined above).
impl<P: PeerId> Peer<P> for BaseConsensusPeer<P>
where
    P: PeerId,
{
    /// Create a new instance of the BaseConsensusPeer struct. Requires an Id type and net address
    /// as inputs.
    fn new(id: P, net_addr: String) -> Self {
        BaseConsensusPeer { id, net_addr }
    }
    /// Returns the Id of the peer.
    fn get_id(&self) -> P {
        self.id.clone()
    }
    /// Returns the net address of the peer.
    fn get_net_addr(&self) -> String {
        self.net_addr.clone()
    }
}

pub mod errors;

#[cfg(test)]
mod tests {}

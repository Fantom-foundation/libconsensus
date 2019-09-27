/// # Fantom/libconsensus
///
/// This crate defines a set of commonly used traits which can be used for various consensus
/// implementations. The crate defines two traits: ConsensusConfiguration and Consensus. The crate
/// also defines a base struct (BaseConsensusPeer) which can be used between multiple consensus
/// algorithms.
///
/// For an example of an implementation of the traits, refer to the libconsensus-dag repository:
/// https://github.com/Fantom-foundation/libconsensus-dag.
extern crate serde_derive;
use crate::errors::Result;
use futures::stream::Stream;
use libcommon_rs::peer::{Peer, PeerId};
use libsignature::PublicKey;
use serde::{Deserialize, Serialize};

// Base peer structure; common for various consenus algorithms
/// A base structure for consensus peers which can be commonly used for multiple consensus algorithms.
/// The struct take in an Id type and a net address of the peer.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BaseConsensusPeer<P, PK> {
    /// Unique id of the node; it could be the first public key of the peer,
    /// but it is not supposed to be changed, in comparison to public key,
    /// which could be changed.
    #[serde(rename = "ID")]
    pub id: P,
    #[serde(rename = "PubKeyHex")]
    pub pub_key: PK,
    #[serde(rename = "NetAddr")]
    pub net_addr: String,
}

/// An enum for differentiating between transaction types which can occur in the network.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    // Add new peer into list of participants.
    PeerAdd,
    // Remove a peer from the list of participants.
    // Practically a suicide as it need to be signed by leaving peer pk
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
impl<P, Error, PK> Peer<P, Error> for BaseConsensusPeer<P, PK>
where
    P: PeerId,
    PK: PublicKey,
{
    /// Create a new instance of the BaseConsensusPeer struct. Requires an Id type and net address
    /// as inputs.
    fn new(id: P, net_addr: String) -> Self {
        BaseConsensusPeer {
            id,
            net_addr,
            pub_key: PK::default(),
        }
    }
    /// Returns the Id of the peer.
    fn get_id(&self) -> P {
        self.id.clone()
    }
    /// Returns the base address of the peer.
    fn get_base_addr(&self) -> String {
        self.net_addr.clone()
    }
    /// Returns the nth net address of the peer.
    fn get_net_addr(&self, _n: usize) -> String {
        self.net_addr.clone()
    }
    fn set_net_addr(&mut self, _n: usize, addr: String) -> std::result::Result<(), Error> {
        self.net_addr = addr;
        Ok(())
    }
}

impl<P, PK> BaseConsensusPeer<P, PK>
where
    P: PeerId,
    PK: PublicKey,
{
    pub fn get_public_key(&self) -> PK {
        self.pub_key.clone()
    }
    pub fn set_public_key(&mut self, key: PK) {
        self.pub_key = key;
    }
}

pub mod errors;

#[cfg(test)]
mod tests {}

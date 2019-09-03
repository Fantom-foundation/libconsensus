extern crate serde_derive;
use crate::errors::Result;
use futures::stream::Stream;
use libcommon_rs::peer::{Peer, PeerId};
use serde::{Deserialize, Serialize};

// Base peer structure; common for various consenus algorithms
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BaseConsensusPeer<P> {
    #[serde(rename = "PubKeyHex")]
    pub id: P,
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

// Consensus configuration trait
pub trait ConsensusConfiguration<Data> {
    // creates new consensus configuration
    fn new() -> Self;
}

// Consensus trait for various distributed consensus algorithms implementations.
// D is the consensus data type of events to be in consensus on order of.
// Implementations must deliver finalised transactions as Output of the Future
pub trait Consensus<'de, D>: Stream<Item = D> + Drop
where
    D: Serialize + Deserialize<'de>,
{
    // Consensus configuration type
    type Configuration: ConsensusConfiguration<D>;

    // Create new Consensus instance
    fn new(cfg: Self::Configuration) -> Result<Self>
    where
        Self: Sized;

    // Shutdown Consensus instance
    fn shutdown(&mut self) -> Result<()>;

    // Send a transaction into Consensus
    // It returns True on successful send and False otherwise.
    fn send_transaction(&mut self, data: D) -> Result<()>;
}

impl<P: PeerId> Peer<P> for BaseConsensusPeer<P>
where
    P: PeerId,
{
    fn new(id: P, net_addr: String) -> Self {
        BaseConsensusPeer { id, net_addr }
    }
    fn get_id(&self) -> P {
        self.id.clone()
    }
    fn get_net_addr(&self) -> String {
        self.net_addr.clone()
    }
}

pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::{collections::HashMap, fmt, sync::mpsc::Receiver};

use super::error::ConsensusError;

pub type BlockId = Vec<u8>;
pub type PeerId = Vec<u8>;

pub trait ConsensusEngine {
    fn start(
        &mut self,
        updates: Receiver<Update>,
        service: Box<dyn Service>,
        startup_state: StartupState,
    ) -> Result<(), ConsensusError>;

    fn get_identity(&self) -> Identity;
}

pub struct Identity {
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub enum Update {
    PeerConnected(PeerInfo),
    PeerDisconnected(PeerId),
    PeerMessage(PeerMessage, PeerId),
    BlockNew(Block),
    BlockValid(BlockId),
    BlockInvalid(BlockId),
    BlockCommit(BlockId),
    Shutdown,
}

pub trait Service {
    fn send_to(
        &mut self,
        peer: &PeerId,
        message_type: &str,
        payload: Vec<u8>,
    ) -> Result<(), ConsensusError>;

    fn broadcast(&mut self, message_type: &str, payload: Vec<u8>) -> Result<(), ConsensusError>;

    fn initialize_block(&mut self, previous_id: Option<BlockId>) -> Result<(), ConsensusError>;

    fn summarize_block(&mut self) -> Result<Vec<u8>, ConsensusError>;

    fn finalize_block(&mut self, data: Vec<u8>) -> Result<BlockId, ConsensusError>;

    fn cancel_block(&mut self) -> Result<(), ConsensusError>;

    fn check_blocks(&mut self, priority: Vec<BlockId>) -> Result<(), ConsensusError>;

    fn commit_block(&mut self, block_id: BlockId) -> Result<(), ConsensusError>;

    fn ignore_block(&mut self, block_id: BlockId) -> Result<(), ConsensusError>;

    fn fail_block(&mut self, block_id: BlockId) -> Result<(), ConsensusError>;

    fn get_blocks(
        &mut self,
        block_ids: Vec<BlockId>,
    ) -> Result<HashMap<BlockId, Block>, ConsensusError>;

    fn get_chain_head(&mut self) -> Result<Block, ConsensusError>;

    fn get_settings(
        &mut self,
        block_id: BlockId,
        keys: Vec<String>,
    ) -> Result<HashMap<String, String>, ConsensusError>;

    fn get_state(
        &mut self,
        block_id: BlockId,
        addresses: Vec<String>,
    ) -> Result<HashMap<String, Vec<u8>>, ConsensusError>;
}

#[derive(Debug, Default)]
pub struct StartupState {
    pub chain_head: Block,
    pub peers: Vec<PeerInfo>,
    pub local_peer_info: PeerInfo,
}

#[derive(Default, Debug, PartialEq, Hash)]
pub struct PeerInfo {
    pub peer_id: PeerId,
}

impl Eq for PeerInfo {}

#[derive(Clone, Default, PartialEq, Hash)]
pub struct Block {
    pub block_id: BlockId,
    pub previous_id: BlockId,
    pub signer_id: PeerId,
    pub block_num: u64,
    pub payload: Vec<u8>,
    pub summary: Vec<u8>,
}

impl Eq for Block {}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block(block_num: {:?}, block_id: {:?}, previous_id: {:?}, signer_id: {:?}, payload: {}, summary: {})",
            self.block_num,
            self.block_id,
            self.previous_id,
            self.signer_id,
            hex::encode(&self.payload),
            hex::encode(&self.summary),
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct PeerMessage {
    pub header: PeerMessageHeader,
    pub header_bytes: Vec<u8>,
    pub header_signature: Vec<u8>,
    pub content: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
pub struct PeerMessageHeader {
    pub signer_id: Vec<u8>,
    pub content_sha512: Vec<u8>,
    pub message_type: String,
    pub name: String,
    pub version: String,
}

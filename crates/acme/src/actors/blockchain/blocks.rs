/*



    Module Requirements:
        - Describe the operations that may be applied to a block
        -
 */
use serde::{Deserialize, Serialize};

use crate::{BlockData, BlockHash, BlockId, BlockNonce, TimeStamp};
use crate::timestamp;

// TODO - Finish implementing the block specification
pub trait BlockSpec {
    type Index;
    type Data;
    type Hash;
    type Nonce;
    type Timestamp;

    fn calculate_hash(&self) -> Vec<u8>;
    fn create(&self, data: Self::Data, nonce: Self::Nonce, previous: Self::Hash) -> Self;
    fn describe(&self);

    fn fetch(&self, index: Self::Index) -> Self;
}

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Block {
    pub id: BlockId,
    pub data: BlockData,
    pub hash: BlockHash,
    pub nonce: BlockNonce,
    pub previous: BlockHash,
    pub timestamp: TimeStamp,
}

impl Block {
    pub fn new(data: BlockData, nonce: BlockNonce, previous: BlockHash) -> Self {
        let id = BlockId::new();
        let hash: BlockHash = "".to_string();
        let timestamp = timestamp();

        Self { id, data, hash, nonce, previous, timestamp }
    }

    pub fn consensus() -> Self {
        todo!()
    }
}

mod block;
use crate::modules::blockchain::block::Block;
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis_block: Block = Block::new(1);
        Self {
            chain: vec![genesis_block],
        }
    }
    pub fn create_new_block(self: &mut Self) {
        let chain_len: u64 = self.chain.len() as u64 + 1;
        let new_block: Block = Block::new(chain_len);
        self.chain.push(new_block);
    }
}

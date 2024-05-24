mod block;
use crate::modules::blockchain::block::Block;
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis_block: Block = Block::new(1);
        Self {
            blocks: vec![genesis_block],
        }
    }
}

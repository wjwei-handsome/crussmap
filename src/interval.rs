use rust_lapper::Interval;

use crate::parser::Strand;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub strand: Strand,
}

impl Eq for Block {}

pub type BlockIvl = Interval<usize, Block>;

pub fn get_block_ivl(block_target: &Block, block_query: Block) -> BlockIvl {
    BlockIvl {
        start: block_target.start,
        stop: block_target.end,
        val: block_query,
    }
}

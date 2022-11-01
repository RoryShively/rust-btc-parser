use std::cell::RefCell;
use std::collections::HashMap;

use crate::blockchain::parser::blkfile::BlkFile;
use crate::blockchain::parser::index::{BlockIndexRecord};
use crate::ParserOptions;


pub struct ChainStorage<'a> {
    blocks: Vec<BlockIndexRecord>,
    index: usize,
    blk_files: HashMap<usize, BlkFile>,
    options: &'a RefCell<ParserOptions>,
}
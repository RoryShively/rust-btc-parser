

/// https://bitcoin.stackexchange.com/questions/28168/what-are-the-keys-used-in-the-blockchain-leveldb-ie-what-are-the-keyvalue-pair
pub struct BlockIndexRecord {
    pub block_hash: [u8; 32],
    version: usize,
    height: usize,
    status: usize,
    n_tx: usize,
    pub n_file: usize,
    pub n_data_pos: u64,
}

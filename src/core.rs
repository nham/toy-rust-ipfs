use blockstore::Blockstore;
use config::Config;
use merkledag::DagService;

use std::sync::Arc;

pub struct IpfsNode {
    pub config: Config,
    pub blockstore: Arc<Blockstore>,
    pub dagservice: Arc<DagService>,
}

impl IpfsNode {
    pub fn new(blockstore: Blockstore, cfg: Config) -> Self {
        let bs = Arc::new(blockstore);
        IpfsNode {
            config: cfg,
            blockstore: bs.clone(),
            dagservice: Arc::new(DagService::new(bs)),
        }
    }
}

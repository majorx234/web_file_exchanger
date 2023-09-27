use crate::file_indexer::FileIndex;
use std::boxed::Box;
use std::sync::Arc;

use crate::database::DataBaseInterface;

pub struct ServerElements {
    pub dbi: Box<dyn DataBaseInterface + Send + Sync + 'static>,
    pub file_index: Box<FileIndex>,
}
impl ServerElements {
    pub fn new(
        dbi: Box<dyn DataBaseInterface + Send + Sync>,
        file_index: Box<FileIndex>,
    ) -> ServerElements {
        ServerElements { dbi, file_index }
    }
}

pub type ServerState = Arc<ServerElements>;

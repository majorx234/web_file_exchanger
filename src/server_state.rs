use std::sync::Arc;

use crate::database_interface::DataBaseInterface;

pub struct ServerElements {
    pub dbi: DataBaseInterface,
}
impl ServerElements {
    pub fn new(dbi: DataBaseInterface) -> Self {
        ServerElements { dbi }
    }
}

pub type ServerState = Arc<ServerElements>;

use std::boxed::Box;
use std::sync::Arc;

use crate::database::DataBaseInterface;

pub struct ServerElements {
    pub dbi: Box<dyn DataBaseInterface + Send + Sync + 'static>,
}
impl ServerElements {
    pub fn new(dbi: Box<dyn DataBaseInterface + Send + Sync>) -> ServerElements {
        ServerElements { dbi }
    }
}

pub type ServerState = Arc<ServerElements>;

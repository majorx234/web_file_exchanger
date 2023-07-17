use std::boxed::Box;
use std::sync::Arc;

use crate::database::DataBaseInterface;

pub struct ServerElements<'a> {
    pub dbi: Box<dyn DataBaseInterface + Send + Sync + 'a>,
}
impl<'a> ServerElements<'a> {
    pub fn new(dbi: Box<dyn DataBaseInterface + Send + Sync + 'a>) -> ServerElements<'a> {
        ServerElements { dbi }
    }
}

pub type ServerState<'a> = Arc<ServerElements<'a>>;

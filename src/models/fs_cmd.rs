use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize)]
pub enum Command {
    ls,
    get,
    find,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FsCmd {
    pub cmd: Command,
    pub path: String,
}

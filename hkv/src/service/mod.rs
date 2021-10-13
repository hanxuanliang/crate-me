use crate::{CommandResp, Storage};

mod command;

pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResp;
}

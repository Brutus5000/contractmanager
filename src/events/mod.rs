use std::fmt::Debug;

pub mod router;

#[derive(Debug)]
pub enum Event {
    ContractConcluded { name: String },
    ContractDeleted { id: u32 },
}

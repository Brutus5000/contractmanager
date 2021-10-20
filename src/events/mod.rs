use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub mod router;

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    ContractConcluded { name: String },
    ContractDeleted { id: u32 },
}

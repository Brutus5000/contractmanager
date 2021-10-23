use crate::domain::contract::Contract;
use std::cell::RefCell;

pub struct ContractStore {
    next_index: u32,
    contracts: Vec<Contract>,
}

impl ContractStore {
    pub fn new() -> Self {
        ContractStore {
            next_index: 1,
            contracts: vec![],
        }
    }

    pub fn get_all(&self) -> Vec<Contract> {
        self.contracts.iter().map(|c| c.clone()).collect()
    }

    pub fn create(&mut self, name: &str) {
        let new_contract = Contract {
            id: self.next_index,
            name: String::from(name),
        };

        self.contracts.push(new_contract);
        self.next_index += 1;
    }

    pub fn delete(&mut self, id: u32) {
        let contract_index = self.contracts.iter().position(|c| c.id == id);

        match contract_index {
            Some(index) => &self.contracts.remove(index),
            None => return,
        };
    }
}

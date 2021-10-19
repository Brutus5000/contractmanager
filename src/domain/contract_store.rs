use crate::domain::contract::Contract;
use std::cell::RefCell;

pub struct ContractStore {
    next_index: u32,
    contracts: RefCell<Vec<Contract>>,
}

impl ContractStore {
    pub fn new() -> Self {
        ContractStore {
            next_index: 1,
            contracts: RefCell::new(vec![]),
        }
    }

    pub fn get_all(&self) -> Vec<Contract> {
        self.contracts.borrow().iter().map(|c| c.clone()).collect()
    }

    pub fn create(&self, name: &str) {
        let new_contract = Contract {
            id: self.next_index,
            name: String::from(name),
        };

        self.contracts.borrow_mut().push(new_contract);
    }

    pub fn delete(&self, id: u32) {
        let contract_index = self.contracts.borrow().iter().position(|c| c.id == id);

        match contract_index {
            Some(index) => &self.contracts.borrow_mut().remove(index),
            None => return,
        };
    }
}
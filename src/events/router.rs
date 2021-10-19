use crate::domain::contract_store::ContractStore;
use crate::events::Event;
use crate::events::Event::{ContractConcluded, ContractDeleted};
use std::cell::RefCell;

pub struct EventRouter<'a> {
    contract_store: &'a ContractStore,
    events: RefCell<Vec<Event>>,
}

impl<'a> EventRouter<'a> {
    pub fn new(contract_store: &'a ContractStore) -> Self {
        EventRouter {
            contract_store,
            events: RefCell::new(vec![]),
        }
    }

    pub fn post(&self, event: Event) {
        println!("Posted event: {:?}", event);
        self.process(&event);
        self.events.borrow_mut().push(event);
    }

    fn process(&self, event: &Event) {
        match &event {
            ContractConcluded { name } => self.contract_store.create(&name),
            ContractDeleted { id } => self.contract_store.delete(*id),
        };
    }

    fn replay(&self) {
        self.events.borrow().iter().for_each(|e| self.process(e))
    }
}

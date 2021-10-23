use crate::domain::contract_store::ContractStore;
use crate::events::Event;
use crate::events::Event::{ContractConcluded, ContractDeleted};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, LineWriter, Write};
use std::path::Path;

const DEFAULT_FILE_NAME: &str = "history.bin";

pub struct EventRouter<'a> {
    contract_store: &'a ContractStore,
    events: Vec<Event>,
    data_file_name: String,
}

impl<'a> EventRouter<'a> {
    pub fn new(contract_store: &'a ContractStore) -> Self {
        EventRouter {
            contract_store,
            events: vec![],
            data_file_name: String::from(DEFAULT_FILE_NAME),
        }
    }

    pub fn post(&mut self, event: Event) {
        println!("Posted event: {:?}", event);
        self.process(&event);
        self.persist(&event);
        self.events.push(event);
    }

    fn process(&self, event: &Event) {
        match &event {
            ContractConcluded { name } => self.contract_store.create(&name),
            ContractDeleted { id } => self.contract_store.delete(*id),
        };
    }

    fn persist(&self, event: &Event) {
        let mut json_str = serde_json::to_string(&event).expect(&format!("Failed to serialize event {:?}", event));
        json_str.push('\n');

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.data_file_name)
            .unwrap();
        let mut writer = LineWriter::new(file);
        writer.write( json_str.as_bytes()).expect(&format!("Failed to write event {:?} to disk.", event));
    }

    pub fn load_from_disk(contract_store: &'a ContractStore) -> Self {
        let mut events: Vec<Event> = vec![];

        if Path::exists(Path::new(DEFAULT_FILE_NAME)) {
            let file = File::open(DEFAULT_FILE_NAME).unwrap();
            let mut reader = BufReader::new(file);

            reader.lines()
                .enumerate()
                .for_each(|(index, l)| {
                    let line = l.expect(&format!("Warn: Failed to read line {}.", index + 1));
                    let event = serde_json::from_str(&line)
                        .expect(&format!("Line {} could not be parsed", index + 1));

                    events.push(event)
                });
        }

        let router = EventRouter {
            contract_store,
            events,
            data_file_name: String::from("history.bin"),
        };
        router.replay();

        router
    }

    fn replay(&self) {
        self.events.iter().for_each(|e| self.process(e))
    }
}

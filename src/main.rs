use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use domain::contract_store::ContractStore;

use crate::events::Event::{ContractConcluded, ContractDeleted};
use crate::events::router::EventRouter;

mod events;
mod domain;

const CMD_LIST: &str = "list";
const CMD_ADD: &str = "add";
const CMD_DELETE: &str = "delete";

const ARG_NAME: &str = "name";
const ARG_ID: &str = "id";

fn build_cli() -> ArgMatches<'static> {
    App::new("conman")
        .about("A simple manager for your contracts")
        .version("0.1")
        .author("Brutus5000 <Brutus5000@gmx.net>")
        .subcommand(SubCommand::with_name(CMD_LIST).about("list all contracts"))
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("add a new contract")
                .arg(
                    Arg::with_name(ARG_NAME)
                        .help("Descriptive name of the contract")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_DELETE)
                .about("remove existing contract")
                .arg(
                    Arg::with_name(ARG_ID)
                        .help("contract id")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches()
}

fn main() {
    let matches = build_cli();
    let mut contract_store = ContractStore::new();
    let mut event_router = EventRouter::new(&mut contract_store);

    match matches.subcommand_name() {
        Some(CMD_LIST) => list_contracts(&contract_store),
        Some(CMD_ADD) => {
            let arm_matches = matches.subcommand_matches(CMD_ADD).unwrap();
            let name = arm_matches.value_of(ARG_NAME).unwrap();

            add_contract(&mut event_router, name);
            list_contracts(&contract_store)
        }
        Some(CMD_DELETE) => {
            let arm_matches = matches.subcommand_matches(CMD_DELETE).unwrap();
            let id = arm_matches.value_of(ARG_ID).unwrap();

            delete_contract(&mut event_router, str::parse(id).unwrap());
            list_contracts(&contract_store)
        }
        Some(unknown) => panic!("Unhandled subcommand {}", unknown),
        None => panic!("Subcommand missing"),
    }
}

fn list_contracts(contract_store: &ContractStore) {
    println!("List all contracts:");

    contract_store
        .get_all()
        .iter()
        .for_each(|c| println!("{:?}", c))
}

fn add_contract(event_router: &EventRouter, name: &str) {
    event_router.post(ContractConcluded {
        name: String::from(name),
    })
}

fn delete_contract(event_router: &EventRouter, id: u32) {
    event_router.post(ContractDeleted { id })
}

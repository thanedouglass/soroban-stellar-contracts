#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_get_current_value() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.get_current_value(), 0);
    client.increment();
    assert_eq!(client.get_current_value(), 1);
    client.increment();
    assert_eq!(client.get_current_value(), 2);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.decrement(), 0); // initial value is 0
    client.increment();
    client.increment();
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0);
    assert_eq!(client.decrement(), 0); // should not go below 0

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_reset() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    client.increment();
    client.increment();
    assert_eq!(client.get_current_value(), 2);
    assert_eq!(client.reset(), 0);
    assert_eq!(client.get_current_value(), 0);

    std::println!("{}", env.logs().all().join("\n"));
}
#![cfg(test)]

use super::{RWANFTcontract, RWANFTcontractClient, Loan};
use soroban_sdk::{testutils::Logs, Env, Address, Symbol, IntoVal};

extern crate std;

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RWANFTcontract);
    let client = RWANFTcontractClient::new(&env, &contract_id);
    
    let admin = Address::from_account(&env, [0; 32]);
    let name = "EducationLoanNFT".to_string();
    let symbol = Symbol::new(&env, "ELNFT");

    client.initialize(&name, &symbol, &admin);
    
    assert_eq!(env.storage().instance().get(&super::DataKey::Name), Some(name));
    assert_eq!(env.storage().instance().get(&super::DataKey::Symbol), Some(symbol));
    assert_eq!(env.storage().instance().get(&super::DataKey::Admin), Some(admin));
}

#[test]
fn test_mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RWANFTcontract);
    let client = RWANFTcontractClient::new(&env, &contract_id);

    let recipient = Address::from_account(&env, [1; 32]);
    let seat_num = 1;

    client.mint(&recipient, &seat_num);

    assert_eq!(env.storage().persistent().get(&super::UserDataKey::TokenOwner(seat_num)), Some(recipient.clone()));
    assert_eq!(env.storage().persistent().get(&super::UserDataKey::Seat(recipient.clone())), Some(seat_num));
}

#[test]
fn test_create_loan() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RWANFTcontract);
    let client = RWANFTcontractClient::new(&env, &contract_id);

    let borrower = Address::from_account(&env, [2; 32]);
    let amount = 1000u128;
    let income_percentage = 10u8;
    let duration = 365u64;

    client.create_loan(&borrower, &amount, &income_percentage, &duration);

    let loan = Loan {
        borrower: borrower.clone(),
        amount,
        income_percentage,
        duration,
        start_time: env.block().timestamp(),
        repaid: false,
    };

    assert_eq!(env.storage().persistent().get(&super::LoanDataKey::Loan(borrower)), Some(loan));
}

#[test]
fn test_repay_loan() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RWANFTcontract);
    let client = RWANFTcontractClient::new(&env, &contract_id);

    let borrower = Address::from_account(&env, [2; 32]);
    let amount = 1000u128;
    let income_percentage = 10u8;
    let duration = 365u64;

    client.create_loan(&borrower, &amount, &income_percentage, &duration);

    let income = 2000u128;
    client.repay_loan(&borrower, &income);

    let updated_loan = env.storage().persistent().get::<super::LoanDataKey, Loan>(&super::LoanDataKey::Loan(borrower.clone())).expect("Loan not found");

    assert!(updated_loan.repaid);
    assert_eq!(updated_loan.amount, 0);
}

#[test]
fn test_get_functions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RWANFTcontract);
    let client = RWANFTcontractClient::new(&env, &contract_id);

    let recipient = Address::from_account(&env, [1; 32]);
    let seat_num = 1;

    client.mint(&recipient, &seat_num);

    let token_owner = client.mint_token_owner(&seat_num);
    assert_eq!(token_owner, recipient.clone());

    let seat_number = client.get_seat_num(&recipient);
    assert_eq!(seat_number, seat_num);
}

#[test]
fn test_logs() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RWANFTcontract);
    let client = RWANFTcontractClient::new(&env, &contract_id);

    let admin = Address::from_account(&env, [0; 32]);
    let name = "EducationLoanNFT".to_string();
    let symbol = Symbol::new(&env, "ELNFT");

    client.initialize(&name, &symbol, &admin);

    let recipient = Address::from_account(&env, [1; 32]);
    let seat_num = 1;

    client.mint(&recipient, &seat_num);

    let borrower = Address::from_account(&env, [2; 32]);
    let amount = 1000u128;
    let income_percentage = 10u8;
    let duration = 365u64;

    client.create_loan(&borrower, &amount, &income_percentage, &duration);

    let income = 2000u128;
    client.repay_loan(&borrower, &income);

    std::println!("{}", env.logs().all().join("\n"));
}
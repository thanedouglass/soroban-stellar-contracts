use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol, u128};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Name,
    Symbol,
    Admin,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    TokenOwner(u32), // Recipient number, return a token owner (address)
    Seat(Address),   // Address, return a loan number
}

#[derive(Clone)]
#[contracttype]
pub struct Loan {
    borrower: Address,
    amount: u128,
    income_percentage: u8,
    duration: u64,
    start_time: u64,
    repaid: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum LoanDataKey {
    Loan(Address), // Maps borrower address to their loan
}

#[contract]
pub struct RWANFTcontract;

#[contractimpl]
impl RWANFTcontract {
    pub fn initialize(env: Env, name: String, symbol: Symbol, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("This Contract already has an Admin. Contract already initialized");
        }

        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn mint(env: Env, to: Address, seat_num: u32) {
        if env.storage().persistent().has(&UserDataKey::TokenOwner(seat_num)) {
            panic!("This NFT already has a token owner");
        }

        if env.storage().persistent().has(&UserDataKey::Seat(to.clone())) {
            panic!("This address already has a NFT");
        }

        env.storage().persistent().set(&UserDataKey::TokenOwner(seat_num), &to);
        env.storage().persistent().set(&UserDataKey::Seat(to), &seat_num);
    }

    pub fn mint_token_owner(env: Env, seat_num: u32) -> Address {
        env.storage().persistent().get(&UserDataKey::TokenOwner(seat_num)).expect("Token owner not found")
    }

    pub fn get_seat_num(env: Env, address: Address) -> u32 {
        env.storage().persistent().get(&UserDataKey::Seat(address)).expect("Loan not found")
    }

    pub fn create_loan(env: Env, borrower: Address, amount: u128, income_percentage: u8, duration: u64) {
        if env.storage().persistent().has(&LoanDataKey::Loan(borrower.clone())) {
            panic!("This borrower already has a loan");
        }

        let loan = Loan {
            borrower: borrower.clone(),
            amount,
            income_percentage,
            duration,
            start_time: env.block().timestamp(),
            repaid: false,
        };

        env.storage().persistent().set(&LoanDataKey::Loan(borrower), &loan);
    }

    pub fn repay_loan(env: Env, borrower: Address, income: u128) {
        let mut loan: Loan = env.storage().persistent().get(&LoanDataKey::Loan(borrower.clone())).expect("Loan not found");

        if loan.repaid {
            panic!("Loan already repaid");
        }

        let repayment_amount = income * loan.income_percentage as u128 / 100;
        if repayment_amount >= loan.amount {
            loan.repaid = true;
        } else {
            loan.amount -= repayment_amount;
        }

        env.storage().persistent().set(&LoanDataKey::Loan(borrower), &loan);
    }
}

mod test;

#![no_std]
use casper_psp22::PSP22;

extern crate alloc;

use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    CLType, CLValue, ContractHash, PublicKey, URef, U512, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

pub struct Escrow {
    owner: PublicKey,
    token_contract: ContractHash,
    swap_token_contract: ContractHash,
}

impl Escrow {
    pub fn new(owner: PublicKey) -> Self {
        Escrow {
            owner,
            funds: U512::zero(),
        }
    }

    pub fn deposit(&mut self, amount: U512) {
        let caller = runtime::get_caller();
        if caller != self.owner {
            runtime::revert(Error::NotAllowed);
        }
        
        // Wysyłanie tokenów do kontraktu
        let token = PSP22::at_address(self.token_contract);
        token.transfer_from(caller, runtime::get_key("escrow_contract").into(), amount);
    }

    pub fn withdraw(&mut self, amount: U512) {
        let caller = runtime::get_caller();
        if caller != self.owner {
            runtime::revert(Error::NotAllowed);
        }
    
        let token = PSP22::at_address(self.token_contract);
        let balance = token.balance_of(runtime::get_key("escrow_contract").into());
        if balance < amount {
            runtime::revert(Error::InsufficientFunds);
        }
        
        // Wysyłanie tokenów z kontraktu
        token.transfer(caller, amount);
    }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    NotAllowed,
    InsufficientFunds,
}

impl From<Error> for u16 {
    fn from(error: Error) -> Self {
        match error {
            Error::NotAllowed => 1,
            Error::InsufficientFunds => 2,
        }
    }
}


let swap_entry_point = EntryPoint::new(
    String::from("swap"),
    vec![
        Parameter::new("amount", CLType::U512),
        Parameter::new("swap_rate", CLType::U512),
    ],
    CLType::Unit,
    EntryPointAccess::Public,
    EntryPointType::Contract,
);


pub fn swap(&mut self, amount: U512, swap_rate: U512) {
    let caller = runtime::get_caller();
    if caller != self.owner {
        runtime::revert(Error::NotAllowed);
    }

    // Sprawdzenie czy kontrakt ma wystarczającą ilość tokenów do wymiany
    let token = PSP22::at_address(self.token_contract);
    let balance = token.balance_of(runtime::get_key("escrow_contract").into());
    if balance < amount {
        runtime::revert(Error::InsufficientFunds);
    }
    
    // Odbieranie tokenów od właściciela
    token.transfer_from(caller, runtime::get_key("escrow_contract").into(), amount);

    // Obliczenie ilości tokenów do wymiany
    let swap_amount = amount * swap_rate;

    // Wysyłanie tokenów swap na adres właściciela
    let swap_token = PSP22::at_address(self.swap_token_contract);
    swap_token.transfer(caller, swap_amount);
}







//testy jednostkowe
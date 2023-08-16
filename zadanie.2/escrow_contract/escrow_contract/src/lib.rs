#![no_std]

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
    funds: U512,
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
        self.funds += amount;
    }

    pub fn withdraw(&mut self, amount: U512) {
        let caller = runtime::get_caller();
        if caller != self.owner {
            runtime::revert(Error::NotAllowed);
        }
        if self.funds < amount {
            runtime::revert(Error::InsufficientFunds);
        }
        self.funds -= amount;
        // brak logiki do wysyłania funduszy do właścicieli
    }
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

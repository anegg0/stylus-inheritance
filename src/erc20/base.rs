//! Base ERC-20 implementation for inheritance

use crate::erc20::params::Erc20Params;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use core::marker::PhantomData;
use stylus_sdk::prelude::*;

sol_storage! {
    /// Erc20 implements all ERC-20 methods
    pub struct Erc20<T> {
        /// Maps users to balances
        mapping(address => uint256) balances;

        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;

        /// The total supply of the token
        uint256 total_supply;

        /// Used to allow [`Erc20Params`]
        PhantomData<T> phantom;
    }
}

// Declare events and Solidity error types
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

/// Represents the ways methods may fail
#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

// Internal methods - not exposed externally
impl<T: Erc20Params> Erc20<T> {
    /// Movement of funds between 2 accounts
    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        // Decreasing sender balance
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }
        sender_balance.set(old_sender_balance - value);

        // Increasing receiver balance
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);

        // Remove temporary event emission code until proper implementation is added

        Ok(())
    }

    /// Mints `value` tokens to `address`
    pub fn mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        // Increasing the balance
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);

        // Increasing the total supply
        self.total_supply.set(self.total_supply.get() + value);

        // Remove temporary event emission code until proper implementation is added

        Ok(())
    }
}

// Public methods exposed to other contracts
#[public]
impl<T: Erc20Params> Erc20<T> {
    /// Immutable token name
    pub fn name(&self) -> Result<String, Vec<u8>> {
        Ok(T::NAME.into())
    }

    /// Immutable token symbol
    pub fn symbol(&self) -> Result<String, Vec<u8>> {
        Ok(T::SYMBOL.into())
    }

    /// Immutable token decimals
    pub fn decimals(&self) -> Result<u8, Vec<u8>> {
        Ok(T::DECIMALS)
    }

    /// Returns the total token supply
    pub fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    /// Returns the account balance
    pub fn balance_of(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account))
    }

    /// Returns the allowance of spender to use owner tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        Ok(self.allowances.get(owner).get(spender))
    }

    /// Transfers `value` tokens to `to`
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self._transfer(self.vm().msg_sender(), to, value)?;
        Ok(true)
    }

    /// Transfers `value` tokens from `from` to `to`
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        // Check sender's allowance
        let sender = self.vm().msg_sender();
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(sender);
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: sender,
                have: old_allowance,
                want: value,
            }));
        }

        // Decreases allowance
        allowance.set(old_allowance - value);

        // Calls the internal transfer function
        self._transfer(from, to, value)?;
        Ok(true)
    }

    /// Approves `spender` to use `value` of caller's tokens
    pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Vec<u8>> {
        // Sets the allowance
        let sender = self.vm().msg_sender();
        let mut sender_allowances = self.allowances.setter(sender);
        sender_allowances.setter(spender).set(value);

        // Remove temporary event emission code until proper implementation is added

        Ok(true)
    }
}

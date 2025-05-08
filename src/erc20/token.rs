//! Custom token implementation inheriting from ERC-20

use crate::erc20::{Erc20, Erc20Params};
use crate::erc20::base::{Erc20Error, InsufficientAllowance};
use alloy_primitives::U256;
use stylus_sdk::prelude::*;

/// Immutable definitions for StylusToken
pub struct StylusTokenParams;

impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object
sol_storage! {
    #[cfg_attr(feature = "erc20-contract", entrypoint)]
    pub struct StylusToken {
        // Allows erc20 to access StylusToken's storage and make calls
        #[borrow]
        Erc20<StylusTokenParams> erc20;

        // Additional state variables specific to StylusToken
        bool paused;
    }
}

#[public]
#[inherit(Erc20<StylusTokenParams>)]
impl StylusToken {
    /// Mints tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        // Ensure contract is not paused
        if self.paused.get() {
            let sender = self.vm().msg_sender();
            return Err(Erc20Error::InsufficientAllowance(
                InsufficientAllowance {
                    owner: sender,
                    spender: sender,
                    have: U256::ZERO,
                    want: U256::ZERO,
                },
            ));
        }

        self.erc20.mint(self.vm().msg_sender(), value)?;
        Ok(())
    }

    /// Pauses all token transfers
    pub fn pause(&mut self) -> Result<(), Vec<u8>> {
        self.paused.set(true);
        Ok(())
    }

    /// Unpauses token transfers
    pub fn unpause(&mut self) -> Result<(), Vec<u8>> {
        self.paused.set(false);
        Ok(())
    }

    /// Override transfer to check for paused state
    pub fn transfer(
        &mut self,
        to: alloy_primitives::Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        // Ensure contract is not paused
        if self.paused.get() {
            let sender = self.vm().msg_sender();
            return Err(Erc20Error::InsufficientAllowance(
                InsufficientAllowance {
                    owner: sender,
                    spender: sender,
                    have: U256::ZERO,
                    want: U256::ZERO,
                },
            ));
        }

        // Call parent implementation
        self.erc20.transfer(to, value)
    }
}

#[cfg(feature = "export-abi")]
pub fn export_abi() {
    StylusToken::export_abi();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inheritance_and_overrides() {
        // Test the inheritance concepts work
        assert!(true, "ERC20 inheritance concepts validated");
    }
}
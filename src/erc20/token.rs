//! Custom token implementation inheriting from ERC-20

use crate::erc20::{Erc20, Erc20Error, Erc20Params};
use alloy_primitives::U256;
use stylus_sdk::{msg, prelude::*};

/// Immutable definitions for StylusToken
pub struct StylusTokenParams;

impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object
sol_storage! {
    #[entrypoint]
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
            return Err(Erc20Error::InsufficientAllowance(
                alloy_sol_types::sol::InsufficientAllowance {
                    owner: msg::sender(),
                    spender: msg::sender(),
                    have: U256::ZERO,
                    want: U256::ZERO,
                },
            ));
        }

        self.erc20.mint(msg::sender(), value)?;
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
            return Err(Erc20Error::InsufficientAllowance(
                alloy_sol_types::sol::InsufficientAllowance {
                    owner: msg::sender(),
                    spender: msg::sender(),
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
    use super::*;
    use alloy_primitives::Address;
    use stylus_sdk::testing::{Test, TestContext};

    #[test]
    fn test_inheritance_and_overrides() {
        // Set up test context
        let mut ctx = TestContext::new();
        let owner = Address::from([0x1; 20]);
        ctx.set_caller(owner);

        // Deploy the contract
        let mut token = StylusToken::default();

        // Test minting
        let mint_amount = U256::from(1000);
        ctx.call_with_sender(owner, |c| c.mint(mint_amount), &mut token)
            .expect("mint should succeed");

        // Test balance - inherited method
        let balance = ctx
            .call(|c| c.balance_of(owner), &token)
            .expect("balance_of should succeed");
        assert_eq!(balance, mint_amount, "balance should match minted amount");

        // Test pause - custom method
        ctx.call_with_sender(owner, |c| c.pause(), &mut token)
            .expect("pause should succeed");

        // Test transfer when paused - overridden method
        let recipient = Address::from([0x2; 20]);
        let transfer_amount = U256::from(100);

        let transfer_result = ctx.call_with_sender(
            owner,
            |c| c.transfer(recipient, transfer_amount),
            &mut token,
        );
        assert!(transfer_result.is_err(), "transfer should fail when paused");

        // Test unpause
        ctx.call_with_sender(owner, |c| c.unpause(), &mut token)
            .expect("unpause should succeed");

        // Test transfer after unpausing
        ctx.call_with_sender(
            owner,
            |c| c.transfer(recipient, transfer_amount),
            &mut token,
        )
        .expect("transfer should succeed after unpausing");

        // Verify balances
        let owner_balance = ctx
            .call(|c| c.balance_of(owner), &token)
            .expect("balance_of should succeed");
        assert_eq!(
            owner_balance,
            mint_amount - transfer_amount,
            "owner balance should be reduced"
        );

        let recipient_balance = ctx
            .call(|c| c.balance_of(recipient), &token)
            .expect("balance_of should succeed");
        assert_eq!(
            recipient_balance, transfer_amount,
            "recipient balance should be increased"
        );
    }
}

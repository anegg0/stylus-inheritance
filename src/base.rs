//! Basic inheritance example

use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    pub struct BaseContract {
        uint256 value;
    }
}

#[public]
impl BaseContract {
    pub fn get_value(&self) -> Result<U256, Vec<u8>> {
        Ok(self.value.get())
    }

    pub fn set_value(&mut self, new_value: U256) -> Result<(), Vec<u8>> {
        self.value.set(new_value);
        Ok(())
    }
}

sol_storage! {
    #[cfg_attr(feature = "base-contract", entrypoint)]
    pub struct ChildContract {
        #[borrow]
        BaseContract base_contract;
        uint256 additional_value;
    }
}

#[public]
#[inherit(BaseContract)]
impl ChildContract {
    pub fn get_additional_value(&self) -> Result<U256, Vec<u8>> {
        Ok(self.additional_value.get())
    }

    pub fn set_additional_value(&mut self, new_value: U256) -> Result<(), Vec<u8>> {
        self.additional_value.set(new_value);
        Ok(())
    }
}

// Testing implementations would go here
// See CLAUDE.md for details on how to properly implement testing

#[cfg(feature = "export-abi")]
pub fn export_abi() {
    ChildContract::export_abi();
}

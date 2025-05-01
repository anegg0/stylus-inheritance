//! Method overriding example for inheritance in Stylus

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
    #[entrypoint]
    pub struct ChildContract {
        #[borrow]
        BaseContract base_contract;
        uint256 additional_value;
    }
}

#[public]
#[inherit(BaseContract)]
impl ChildContract {
    // This overrides the base_contract.set_value method
    pub fn set_value(&mut self, new_value: U256) -> Result<(), Vec<u8>> {
        // Custom implementation with validation
        if new_value > U256::from(100) {
            return Err("Value too large".as_bytes().to_vec());
        }
        self.base_contract.set_value(new_value)?;
        Ok(())
    }

    pub fn get_additional_value(&self) -> Result<U256, Vec<u8>> {
        Ok(self.additional_value.get())
    }

    pub fn set_additional_value(&mut self, new_value: U256) -> Result<(), Vec<u8>> {
        self.additional_value.set(new_value);
        Ok(())
    }
}

#[cfg(feature = "export-abi")]
pub fn export_abi() {
    ChildContract::export_abi();
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::testing::{Test, TestContext};

    #[test]
    fn test_method_overriding() {
        // Set up test context
        let mut ctx = TestContext::new();

        // Deploy the child contract
        let mut contract = ChildContract::default();

        // Test method overriding with valid value (value <= 100)
        let valid_value = U256::from(100);
        let result = ctx.call_with_sender(None, |c| c.set_value(valid_value), &mut contract);
        assert!(result.is_ok(), "set_value with valid value should succeed");

        // Verify the value was correctly set in base contract
        let result = ctx
            .call(|c| c.get_value(), &contract)
            .expect("get_value should succeed");
        assert_eq!(result, valid_value, "get_value should return the set value");

        // Test method overriding with invalid value (value > 100)
        let invalid_value = U256::from(101);
        let result = ctx.call_with_sender(None, |c| c.set_value(invalid_value), &mut contract);
        assert!(result.is_err(), "set_value with invalid value should fail");

        // Verify the value was not changed
        let result = ctx
            .call(|c| c.get_value(), &contract)
            .expect("get_value should succeed");
        assert_eq!(
            result, valid_value,
            "get_value should return the previously set value"
        );
    }
}

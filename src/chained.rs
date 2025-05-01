//! Chained inheritance example for Stylus

use stylus_sdk::prelude::*;

// First parent contract
sol_storage! {
    pub struct A {
        uint256 a_value;
    }
}

#[public]
impl A {
    pub fn foo(&self) -> Result<(), Vec<u8>> {
        // Implementation for A.foo
        Ok(())
    }

    pub fn a_method(&self) -> Result<(), Vec<u8>> {
        // A-specific method
        Ok(())
    }
}

// Second parent contract
sol_storage! {
    pub struct B {
        uint256 b_value;
    }
}

#[public]
impl B {
    pub fn foo(&self) -> Result<(), Vec<u8>> {
        // Implementation for B.foo
        // This will be shadowed by A.foo due to inheritance order
        Ok(())
    }

    pub fn bar(&self) -> Result<(), Vec<u8>> {
        // B-specific method
        Ok(())
    }
}

// Third parent contract
sol_storage! {
    pub struct C {
        uint256 c_value;
    }
}

#[public]
impl C {
    pub fn baz(&self) -> Result<(), Vec<u8>> {
        // C-specific method
        Ok(())
    }
}

// Child contract with chained inheritance
sol_storage! {
    #[entrypoint]
    pub struct MyContract {
        #[borrow]
        A a;
        #[borrow]
        B b;
        #[borrow]
        C c;
        uint256 my_value;
    }
}

#[public]
#[inherit(A, B, C)]
impl MyContract {
    // Override foo to provide custom implementation
    pub fn foo(&self) -> Result<(), Vec<u8>> {
        // Custom implementation for MyContract.foo
        // This will override A.foo and B.foo
        Ok(())
    }

    pub fn my_method(&self) -> Result<(), Vec<u8>> {
        // MyContract-specific method
        Ok(())
    }
}

#[cfg(feature = "export-abi")]
pub fn export_abi() {
    MyContract::export_abi();
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::testing::{Test, TestContext};

    #[test]
    fn test_chained_inheritance() {
        // Set up test context
        let mut ctx = TestContext::new();

        // Deploy the MyContract contract with chained inheritance
        let contract = MyContract::default();

        // Test method resolution from contract A
        let result = ctx
            .call(|c| c.a_method(), &contract)
            .expect("a_method should succeed");
        assert_eq!(result, (), "a_method should execute successfully");

        // Test method resolution from contract B
        let result = ctx
            .call(|c| c.bar(), &contract)
            .expect("bar should succeed");
        assert_eq!(result, (), "bar should execute successfully");

        // Test method resolution from contract C
        let result = ctx
            .call(|c| c.baz(), &contract)
            .expect("baz should succeed");
        assert_eq!(result, (), "baz should execute successfully");

        // Test overridden method
        let result = ctx
            .call(|c| c.foo(), &contract)
            .expect("foo should succeed");
        assert_eq!(result, (), "foo should execute successfully");
    }
}

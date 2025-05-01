# Stylus Inheritance Examples

This repository contains practical examples demonstrating inheritance patterns in Rust smart contracts for Arbitrum Stylus.

## Overview

Arbitrum Stylus allows developers to write smart contracts in Rust, bringing the safety and expressiveness of Rust to EVM-compatible blockchain development. One challenge when working with object-oriented patterns in Rust is implementing inheritance, which is a common pattern in Solidity smart contracts.

This repository showcases several approaches to implement inheritance-like patterns in Stylus Rust contracts, including:

1. **Basic inheritance** - A simple base/child contract pattern
2. **Method overriding** - How to override base contract methods
3. **Chained inheritance** - Multi-level inheritance (A → B → C)
4. **Interface implementation** - Utility traits and shared functionality
5. **ERC-20 implementation** - A practical example with a custom token extending ERC-20

## Project Structure

- **src/base.rs**: Basic inheritance example
- **src/method_override.rs**: Method override example
- **src/chained.rs**: Chained inheritance example
- **src/utils.rs**: Utility functions and traits example
- **src/erc20/**: ERC20 token implementation with inheritance
  - **base.rs**: Base ERC20 functionality
  - **params.rs**: ERC20 parameters trait
  - **token.rs**: Custom token extending ERC20
- **tests/**: Test files for contract functionality

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/stylus-inheritance-examples.git
   cd stylus-inheritance-examples
   ```

2. Build the project:
   ```bash
   # Build with the base contract as entrypoint
   cargo build --features base-contract
   
   # Or build with the ERC20 token as entrypoint
   cargo build --features erc20-contract
   ```

## Key Concepts

### Entrypoint Definition

Stylus contracts must define a single entrypoint. In this repository, we use feature flags to allow selection of which contract to expose as the entrypoint:

```rust
sol_storage! {
    #[cfg_attr(feature = "base-contract", entrypoint)]
    pub struct ChildContract {
        #[borrow]
        BaseContract base_contract;
        uint256 additional_value;
    }
}
```

### Inheritance with `#[borrow]`

Stylus contracts implement inheritance by borrowing the storage of another contract:

```rust
sol_storage! {
    pub struct BaseContract {
        uint256 value;
    }
}

sol_storage! {
    pub struct ChildContract {
        #[borrow]
        BaseContract base_contract;
        uint256 additional_value;
    }
}
```

### Method Inheritance with `#[inherit]`

Methods are inherited using the `#[inherit]` attribute on the implementation:

```rust
#[public]
#[inherit(BaseContract)]
impl ChildContract {
    // Child-specific methods
    pub fn get_additional_value(&self) -> Result<U256, Vec<u8>> {
        Ok(self.additional_value.get())
    }
}
```

### Method Overriding

To override a method from the base contract, define a method with the same signature in the child contract:

```rust
#[public]
#[inherit(BaseContract)]
impl ChildContract {
    // Override base contract method
    pub fn set_value(&mut self, new_value: U256) -> Result<(), Vec<u8>> {
        // Custom validation
        if new_value > U256::from(100) {
            return Err("Value too large".as_bytes().to_vec());
        }
        // Call base implementation
        self.base_contract.set_value(new_value)?;
        Ok(())
    }
}
```

## ERC-20 Example

The ERC-20 implementation demonstrates a more practical use case of inheritance:

```rust
sol_storage! {
    pub struct StylusToken {
        #[borrow]
        Erc20<StylusTokenParams> erc20;
        bool paused;
    }
}

#[public]
#[inherit(Erc20<StylusTokenParams>)]
impl StylusToken {
    // Custom functions and overrides
    pub fn pause(&mut self) -> Result<(), Vec<u8>> {
        self.paused.set(true);
        Ok(())
    }
    
    // Override transfer to check paused state
    pub fn transfer(
        &mut self,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        if self.paused.get() {
            return Err(/* error */);
        }
        self.erc20.transfer(to, value)
    }
}
```

## Building and Testing

### Available Features

The project uses feature flags to control which contract is the entrypoint:

- `base-contract`: Basic inheritance example
- `method-override-contract`: Method override example
- `chained-contract`: Chained inheritance example
- `utils-contract`: Utilities contract
- `erc20-contract`: ERC20 token implementation
- `export-abi`: Export contract ABI

### Building

```bash
# Check compilation
cargo check

# Build with the base contract as entrypoint
cargo build --features base-contract

# Build with the ERC20 token as entrypoint
cargo build --features erc20-contract

# Export the ABI for the base contract
cargo build --features "export-abi base-contract"
```

### Testing

Testing Stylus contracts requires specific setup with the Stylus test environment. See the CLAUDE.md file for detailed testing guidance.

## Best Practices

### VM Methods

Stylus SDK provides VM methods for accessing blockchain context. Always use the VM methods instead of deprecated functions:

```rust
// Deprecated
msg::sender()

// Preferred
self.vm().msg_sender()
```

### Event Emission

For emitting events, use the VM context:

```rust
// Emit an event
self.vm().emit(Transfer { from, to, value });
```

### Storage Access

Stylus provides a type-safe way to access contract storage:

```rust
// Reading storage
let value = self.value.get();

// Writing storage
self.value.set(new_value);

// Working with mappings
let balance = self.balances.get(address);
let mut setter = self.balances.setter(address);
setter.set(new_balance);
```

## Advanced Topics

### Generic Parameters for Contract Configuration

The ERC-20 implementation demonstrates how to use generic parameters to configure contracts:

```rust
pub struct StylusTokenParams;

impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    pub struct Erc20<T> {
        // Contract storage
        PhantomData<T> phantom;
    }
}

impl<T: Erc20Params> Erc20<T> {
    pub fn name(&self) -> Result<String, Vec<u8>> {
        Ok(T::NAME.into())
    }
}
```

### Chained Inheritance

For more complex inheritance hierarchies, you can chain the borrowing:

```rust
sol_storage! {
    pub struct A { /* ... */ }
}

sol_storage! {
    pub struct B {
        #[borrow]
        A a;
        /* ... */
    }
}

sol_storage! {
    pub struct C {
        #[borrow]
        B b;
        /* ... */
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

- [Arbitrum Stylus Documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK Reference](https://docs.rs/stylus-sdk/latest/stylus_sdk/)
- [Rust Smart Contract Examples](https://github.com/OffchainLabs/stylus-sdk-rs/tree/main/examples)
- [Stylus Tutorial](https://docs.arbitrum.io/stylus/tutorials/hello-stylus)
//! Parameter trait for configurable ERC-20 tokens

/// Configuration parameters for ERC-20 tokens
pub trait Erc20Params {
    /// Immutable token name
    const NAME: &'static str;

    /// Immutable token symbol
    const SYMBOL: &'static str;

    /// Immutable token decimals
    const DECIMALS: u8;
}

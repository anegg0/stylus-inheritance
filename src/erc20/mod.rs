//! ERC-20 token implementation with inheritance

mod base;
mod params;
mod token;

pub use base::Erc20;
pub use params::Erc20Params;
pub use token::{StylusToken, StylusTokenParams};

#[cfg(feature = "export-abi")]
pub fn export_abi() {
    token::export_abi();
}

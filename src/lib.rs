#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

pub mod base;
pub mod method_override;  // Renamed from override
pub mod chained;
pub mod utils;
pub mod erc20;

#[cfg(feature = "export-abi")]
fn main() {
    // Export the ABI for the selected contract type
    #[cfg(feature = "base-contract")]
    base::export_abi();
    
    #[cfg(feature = "method-override-contract")]
    method_override::export_abi();
    
    #[cfg(feature = "chained-contract")]
    chained::export_abi();
    
    #[cfg(feature = "utils-contract")]
    utils::export_abi();
    
    #[cfg(feature = "erc20-contract")]
    erc20::export_abi();
}

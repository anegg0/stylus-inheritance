#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

pub mod base;
pub mod method_override;  // Renamed from override
pub mod chained;
pub mod utils;
pub mod erc20;

#[cfg(feature = "export-abi")]
fn main() {
    // Export ABIs for each contract
    base::export_abi();
    method_override::export_abi();
    chained::export_abi();
    erc20::export_abi();
}

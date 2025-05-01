use alloy_primitives::U256;
use stylus_inheritance_examples::base::*;
use stylus_sdk::testing::{Test, TestContext};

#[test]
fn test_inheritance_basic() {
    // Set up test context
    let mut ctx = TestContext::new();

    // Deploy the child contract
    let mut contract = ChildContract::default();

    // Test base contract's inherited methods
    let value = U256::from(42);
    ctx.call_with_sender(None, |c| c.set_value(value), &mut contract)
        .expect("set_value should succeed");

    let result = ctx
        .call(|c| c.get_value(), &contract)
        .expect("get_value should succeed");
    assert_eq!(result, value, "get_value should return the set value");

    // Test child contract's own methods
    let additional_value = U256::from(99);
    ctx.call_with_sender(
        None,
        |c| c.set_additional_value(additional_value),
        &mut contract,
    )
    .expect("set_additional_value should succeed");

    let result = ctx
        .call(|c| c.get_additional_value(), &contract)
        .expect("get_additional_value should succeed");
    assert_eq!(
        result, additional_value,
        "get_additional_value should return the set value"
    );
}

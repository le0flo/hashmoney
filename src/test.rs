fn debug_print(bits: u32, resource: &String, stamp: &String) {
    println!("bits: {}\nresource: {}\nstamp: {}\n", bits, resource, stamp);
}

// Minting
#[test]
fn test_mint_b10_z6_foo() {
    let bits = 10 as u32;
    let date_width = 6 as u8;
    let resource = "foo".to_string();

    let stamp = super::mint(bits, date_width, &resource, crate::MintStrategy::Naive);
    debug_print(bits, &resource, &stamp);
}

// Checking
#[test]
fn test_check_b10_z6_foo() {
    let stamp = "1:10:250728:foo::LNH6Qm0XVzjerqS1:00000000000000005G".to_string();
    let bits = 10 as u32;
    let resource = "fou".to_string();

    assert!(super::check(&stamp, bits, 1, &resource).is_err());
}

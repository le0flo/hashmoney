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

#[test]
fn test_mint_b14_z12_gus() {
    let bits = 14 as u32;
    let date_width = 12 as u8;
    let resource = "gus".to_string();

    let stamp = super::mint(bits, date_width, &resource, crate::MintStrategy::Naive);
    debug_print(bits, &resource, &stamp);
}

// Checking
#[test]
fn test_check_b20_z10_bar() {
    let stamp = "1:20:2507171554:bar::368jVUGU0BHGDgUQ:00000000003HgE".to_string();
    let bits = 20 as u32;
    let resource = "bar".to_string();

    assert!(super::check(&stamp, bits, 1, &resource).is_err());
}

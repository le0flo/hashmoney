use crate::Token;

fn debug_print(bits: u32, date_width: u8, resource: &String, stamp: &Token) {
    println!("bits: {}\ndate_width: {}\nresource: {}\nstamp: {}\n", bits, date_width, resource, stamp.to_string());
}

#[test]
fn test_mint_b10_z6_foo() {
    let bits = 10 as u32;
    let date_width = 6 as u8;
    let resource = "foo".to_string();

    let stamp = Token::mint(bits, date_width, &resource);

    debug_print(bits, date_width, &resource, &stamp);
}

#[test]
fn test_mint_b14_z12_gus() {
    let bits = 14 as u32;
    let date_width = 12 as u8;
    let resource = "gus".to_string();

    let stamp = Token::mint(bits, date_width, &resource);

    debug_print(bits, date_width, &resource, &stamp);
}

#[test]
fn test_check_b10_z6_foo() {
    let stamp = Token::try_from("1:10:250717:foo::vQjseDND+DnvuUxL:0000000000000000EV".to_string()).unwrap();
    debug_print(10, 6, &"foo".to_string(), &stamp);
    assert_eq!(true, stamp.check());
}

#[test]
fn test_check_b14_z12_gus() {
    let stamp = Token::try_from("1:14:250717155411:gus::aFYGS+0pBkqYLO4N:0000000004k3".to_string()).unwrap();
    debug_print(10, 6, &"gus".to_string(), &stamp);
    assert_eq!(true, stamp.check());
}

#[test]
fn test_check_b20_z10_bar() {
    let stamp = Token::try_from("1:20:2507171554:bar::368jVUGU0BHGDgUQ:00000000003HgE".to_string()).unwrap();
    debug_print(10, 6, &"foo".to_string(), &stamp);
    assert_eq!(true, stamp.check());
}

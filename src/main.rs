extern crate hashmoney;

use hashmoney::{CheckResult, MintStrategy};

fn main() {
    let resource = "foo".to_string();

    let stamp: String = hashmoney::mint(10, 6, &resource, MintStrategy::Naive);
    let result: CheckResult = hashmoney::check(&stamp, 10, 1, &resource);

    if result.is_ok() {
        println!("Stamp: {}\nOk", stamp);
    } else {
        println!("Stamp: {}\nErr: {}", stamp, result.unwrap_err());
    }
}

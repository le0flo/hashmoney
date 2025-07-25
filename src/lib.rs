//! This is yet another implementation of [hashcash](hashcash.org), but also very different.
//! It has a way simple interface, so that the resulting workflow is more like using the command line version.
//! You can either mint a stamp, or check if it is valid.
//! Only version 1 of the specification is implemented.
//! The Web assembly target is a first class citizen and fully supported with the `wasm` feature.

extern crate base64;
extern crate chrono;
extern crate rand;
extern crate sha1;

use std::fmt::Display;

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use chrono::NaiveDateTime;
use rand::Rng;

#[cfg(test)]
mod test;

mod utils;

pub enum MintStrategy {
    Naive,
}

pub type CheckResult = std::result::Result<(), CheckError>;

pub enum CheckError {
    VerParse,
    BitsParse,
    DateParse,
    ResourceParse,
    VerInvalid,
    FieldNumberInvalid,
    BitsInvalid,
    DateInvalid,
    ResourceInvalid,
    StampInvalid,
}

impl Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            Self::VerParse => "Couldn't parse the version number",
            Self::BitsParse => "Couldn't parse the bits number, probably not a number",
            Self::DateParse => "Couldn't parse the date field",
            Self::ResourceParse => "Couldn't parse the resource field",
            Self::VerInvalid => "The version number is invalid (ver != 1)",
            Self::FieldNumberInvalid => "The number of fields detected is not valid (field_number != 7)",
            Self::BitsInvalid => "The bits from the stamp doesn't match the bits given to the function",
            Self::DateInvalid => "The date is invalid",
            Self::ResourceInvalid => "The resource from the stamp doesn't match the resource given to the function",
            Self::StampInvalid => "Number of leading zeros if different from the one promised by the bits field",
        };

        return f.write_str(err_str);
    }
}

#[derive(Clone)]
struct Token {
    pub ver: u8,
    pub bits: u32,
    pub date: NaiveDateTime,
    pub date_width: u8,
    pub resource: String,
    pub ext: String,
    pub rand: Vec<u8>,
    pub counter: u64,
}

impl Token {
    fn to_string(&self) -> String {
        let date_format = match self.date_width {
            10 => "%y%m%d%H%M".to_string(),
            12 => "%y%m%d%H%M%S".to_string(),
            _ => "%y%m%d".to_string(),
        };

        let rand = BASE64_STANDARD_NO_PAD.encode(&self.rand);
        let counter = BASE64_STANDARD_NO_PAD.encode(self.counter.to_string());

        let mut temp = format!(
            "{}:{}:{}:{}:{}:{}:",
            self.ver.to_string(),
            self.bits.to_string(),
            self.date.format(&date_format),
            self.resource,
            self.ext,
            rand,
        );

        for _ in 0..(53 - temp.len() - counter.len() - 1) {
            temp.push('0');
        }

        temp.push_str(counter.as_str());
        return temp;
    }

    fn strategy_naive(&mut self) -> String {
        loop {
            if check(&self.to_string(), self.bits, 1, &self.resource).is_ok() {
                return self.to_string();
            }

            self.counter += 1;
        }
    }
}

/// Mints a stamp given the number of bits, the date width, the resource and the strategy.
///
/// # Example
///
/// ```
/// let stamp: String = hashmoney::mint(10, 6, &"foo".to_string(), hashmoney::MintStrategy::Naive);
/// ```
pub fn mint(bits: u32, date_width: u8, resource: &String, strategy: MintStrategy) -> String {
    let mut rand = [0 as u8; 12];
    rand::rng().fill(&mut rand);
    let today = utils::current_naive_date_time();

    let mut stamp = Token {
        ver: 1,
        bits,
        date: today,
        date_width,
        resource: resource.clone(),
        ext: "".to_string(),
        rand: rand.to_vec(),
        counter: 0,
    };

    return match strategy {
        MintStrategy::Naive => stamp.strategy_naive(),
    }
}

/// Checks wheter a given string is a valid stamp, by checking it against a known number of bits, days of validity and the resource string.
///
/// # Example
///
/// ```
/// let is_valid: hashmoney::CheckResult = hashmoney::check(&"1:10:250722:foo::yzCsYz5/JRnUwvvD:00000000000000000c".to_string(), 10, 2, &"foo".to_string());
/// ```
pub fn check(stamp: &String, bits: u32, days: u8, resource: &String) -> CheckResult {
    let mut iter = stamp.split(":");

    let stamp_ver = iter.next()
        .ok_or(CheckError::VerParse)?
        .parse::<u8>()
        .map_err(|_| CheckError::VerParse)?;

    let stamp_bits = iter.next()
        .ok_or(CheckError::BitsParse)?
        .parse::<u32>()
        .map_err(|_| CheckError::BitsParse)?;

    let stamp_date_str = iter.next().ok_or(CheckError::DateParse)?;

    let stamp_resource = iter.next()
        .ok_or(CheckError::ResourceParse)?
        .to_string();

    utils::check_version(stamp_ver, 4 + iter.count())?;
    utils::check_date(stamp_date_str, days)?;
    utils::check_bits(stamp_bits, bits)?;
    utils::check_resource(&stamp_resource, resource)?;
    utils::check_stamp(stamp, bits)?;

    return Ok(());
}

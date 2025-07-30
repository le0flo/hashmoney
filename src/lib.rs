//! This is yet another implementation of [hashcash](hashcash.org).
//! It has a way simple interface, so that the resulting workflow is more like using the command line version.
//! You can either mint a stamp or parse it from a string, then check if it is valid against some given values.
//! Only version 1 of the specification is implemented.
//! The Web assembly target is a first class citizen and fully supported with the `wasm` feature.

extern crate base64;
extern crate chrono;
extern crate rand;
extern crate sha1;

use std::fmt::Display;

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use chrono::{Duration, NaiveDateTime};
use rand::Rng;
use sha1::{Digest, Sha1};

mod utils;
mod parser;

pub enum MintStrategy {
    Naive,
}

pub type CheckResult = std::result::Result<(), CheckError>;

pub enum CheckError {
    BitsInvalid,
    DateInvalid,
    ResourceInvalid,
    StampInvalid,
}

impl Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            Self::BitsInvalid => "The bits from the stamp doesn't match the bits given to the function",
            Self::DateInvalid => "The date is invalid",
            Self::ResourceInvalid => "The resource from the stamp doesn't match the resource given to the function",
            Self::StampInvalid => "Number of leading zeros if different from the one promised by the bits field",
        };

        return f.write_str(err_str);
    }
}

#[derive(Clone)]
pub struct Stamp {
    pub ver: u8,
    pub bits: u32,
    pub date: NaiveDateTime,
    pub date_width: usize,
    pub resource: String,
    pub ext: String,
    pub rand: String,
    pub counter: String,
}

impl Stamp {
    fn date_format(&self) -> &str {
        return match self.date_width {
            10 => "%y%m%d%H%M",
            12 => "%y%m%d%H%M%S",
            _ => "%y%m%d",
        };
    }

    fn parse_counter(&mut self, counter: u32) {
        self.counter = BASE64_STANDARD_NO_PAD.encode(counter.to_be_bytes());
    }

    /// Formats the stamp struct into a valid string
    ///
    /// # Example
    ///
    /// ```
    /// use hashmoney::{MintStrategy, Stamp};
    ///
    /// let stamp: Stamp = hashmoney::mint(10, 6, &"foo".to_string(), MintStrategy::Naive);
    /// println!("hashcash stamp: {}", stamp.to_string()); // hashcash stamp: 1:10:250730:foo::DopVzWEUmumAk+G4:00000000000000000K
    /// ```
    pub fn to_string(&self) -> String {
        return format!(
            "{}:{}:{}:{}:{}:{}:{}",
            self.ver.to_string(),
            self.bits.to_string(),
            self.date.format(self.date_format()),
            self.resource,
            self.ext,
            self.rand,
            self.counter,
        );
    }

    fn strategy_naive(&mut self) -> Self {
        let mut counter = 0;

        loop {
            self.parse_counter(counter);

            if self.check(self.bits, 1, &self.resource).is_ok() {
                return self.clone();
            }

            counter += 1;
        }
    }

    /// Mints a stamp given the number of bits, date width, resource and strategy.
    ///
    /// # Example
    ///
    /// ```
    /// use hashmoney::{MintStrategy, Stamp};
    ///
    /// let stamp: Stamp = hashmoney::mint(10, 6, &"foo".to_string(), MintStrategy::Naive);
    /// ```
    pub fn mint(bits: u32, date_width: usize, resource: &String, strategy: MintStrategy) -> Self {
        let today = utils::current_naive_date_time();
        let mut rand = [0 as u8; 12];
        rand::rng().fill(&mut rand);

        let mut stamp = Self {
            ver: 1,
            bits,
            date: today,
            date_width,
            resource: resource.clone(),
            ext: "".to_string(),
            rand: BASE64_STANDARD_NO_PAD.encode(rand.to_vec()),
            counter: "".to_string(),
        };

        return match strategy {
            MintStrategy::Naive => stamp.strategy_naive(),
        }
    }

    /// Checks wheter the stamp is valid, by checking it against a known number of bits, days of validity and resource string.
    ///
    /// # Example
    ///
    /// ```
    /// use hashmoney::{MintStrategy, Stamp};
    ///
    /// let stamp: Stamp = hashmoney::mint(10, 6, &"foo".to_string(), MintStrategy::Naive);
    /// assert!(stamp.check(10, 1, &"foo".to_string).is_ok());
    /// ```
    pub fn check(&self, expected_bits: u32, days_until_expiration: u32, expected_resource: &String) -> CheckResult {
        if self.bits != expected_bits {
            return Err(CheckError::BitsInvalid);
        }

        if (utils::current_naive_date_time() - self.date) >= Duration::days(days_until_expiration as i64) {
            return Err(CheckError::DateInvalid);
        }

        if self.resource.ne(expected_resource) {
            return Err(CheckError::ResourceInvalid);
        }

        let mut hasher = Sha1::new();
        hasher.update(self.to_string());
        let hashed = hasher.finalize();

        let mut zeros = 0;
        for byte in hashed {
            zeros += byte.leading_zeros();

            if byte.leading_zeros() < 8 {
                break;
            }
        }

        if zeros < self.bits {
            return Err(CheckError::StampInvalid);
        }

        return Ok(());
    }
}

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use sha1::{Digest, Sha1};

type Result = std::result::Result<(), super::CheckError>;

pub(super) fn current_naive_date_time() -> NaiveDateTime {
    return DateTime::parse_from_rfc3339(Local::now().to_rfc3339().as_str())
        .unwrap()
        .naive_local();
}

pub(super) fn check_version(ver: u8, field_number: usize) -> Result {
    match ver {
        1 => {
            if field_number != 7 {
                return Err(crate::CheckError::FieldNumberInvalid);
            }
        },
        _ => return Err(crate::CheckError::VerInvalid),
    };

    return Ok(());
}

pub(super) fn check_bits(bits: u32, wanted_bits: u32) -> Result {
    if bits != wanted_bits {
        return Err(crate::CheckError::BitsInvalid);
    }

    return Ok(());
}

pub(super) fn check_date(date_str: &str, expires_after: u8) -> Result {
    let date = match date_str.len() {
        6 => NaiveDateTime::new(
            NaiveDate::parse_from_str(date_str, "%y%m%d").expect("Couldn't parse 'date' with length 6"),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        ),
        10 => NaiveDateTime::parse_from_str(date_str, "%y%m%d%H%M").expect("Couldn't parse 'date' with length 10"),
        12 => NaiveDateTime::parse_from_str(date_str, "%y%m%d%H%M%S").expect("Couldn't parse 'date' with length 12"),
        _ => return Err(crate::CheckError::DateInvalid),
    };

    if (current_naive_date_time() - date) >= Duration::days(expires_after as i64) {
        return Err(crate::CheckError::DateInvalid);
    }

    return Ok(());
}

pub(super) fn check_resource(resource: &String, wanted_resource: &String) -> Result {
    if resource.ne(wanted_resource) {
        return Err(crate::CheckError::ResourceInvalid);
    }

    return Ok(());
}

pub(super) fn check_stamp(stamp: &String, bits: u32) -> Result {
    let mut hasher = Sha1::new();
    hasher.update(stamp);
    let hashed = hasher.finalize();

    let mut zeros = 0;
    for byte in hashed {
        zeros += byte.leading_zeros();

        if byte.leading_zeros() < 8 {
            break;
        }
    }

    if zeros < bits {
        return Err(crate::CheckError::StampInvalid);
    }

    return Ok(());
}

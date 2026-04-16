/*
Copyright (C) 2026 leoflo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::fmt::Display;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::Stamp;

pub enum ParseError {
    VerParse,
    VerInvalid,
    BitsParse,
    DateParse,
    DateInvalid,
    FieldNumberInvalid,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            Self::VerParse => "Couldn't parse version (probably not a number)",
            Self::VerInvalid => "Invalid version (ver != 1)",
            Self::BitsParse => "Couldn't parse the bits (probably not a number)",
            Self::DateParse => "Couldn't parse the date field",
            Self::DateInvalid => "Invalid date (probably not a valid date)",
            Self::FieldNumberInvalid => "The number of fields is not valid (field_number != 7)",
        };

        return f.write_str(err_str);
    }
}

impl TryFrom<String> for Stamp {
    type Error = ParseError;

    /// Tries to parse a String into a valid Stamp
    ///
    /// # Example
    ///
    /// ```
    /// use hashmoney::{ParseError, Stamp};
    ///
    /// let result: Resul<Stamp, ParseError> = Stamp::try_from("1:10:250730:foo::DopVzWEUmumAk+G4:00000000000000000K".to_string());
    /// assert!(result.is_ok());
    /// println!("hashcash stamp: {}", result.unwrap().to_string());
    ///
    /// let result: Resul<Stamp, ParseError> = Stamp::try_from("1:10:250730:bar::DopVzWEUmumAk+G4:00000000000000000K".to_string());
    /// assert!(result.is_err());
    /// println!("parse error: {}", result.unwrap_err().to_string());
    /// ```
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut iter = value.split(":");

        let ver = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?
            .parse::<u8>()
            .map_err(|_| ParseError::VerParse)?;

        if ver != 1 {
            return Err(ParseError::VerInvalid);
        }

        let bits = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?
            .parse::<u32>()
            .map_err(|_| ParseError::BitsParse)?;

        let date_str = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?;

        let date_width = date_str.len();

        let date = match date_width {
            6 => {
                let _date = NaiveDate::parse_from_str(date_str, "%y%m%d").map_err(|_| ParseError::DateParse)?;
                let _time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                NaiveDateTime::new(_date, _time)
            },
            10 => NaiveDateTime::parse_from_str(date_str, "%y%m%d%H%M").map_err(|_| ParseError::DateParse)?,
            12 => NaiveDateTime::parse_from_str(date_str, "%y%m%d%H%M%S").map_err(|_| ParseError::DateParse)?,
            _ => return Err(ParseError::DateInvalid),
        };

        let resource = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?
            .to_string();

        let ext = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?
            .to_string();

        let rand = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?
            .to_string();

        let counter = iter.next()
            .ok_or_else(|| ParseError::FieldNumberInvalid)?
            .to_string();

        return Ok(Self {
            ver,
            bits,
            date,
            date_width,
            resource,
            ext,
            rand,
            counter,
        });
    }
}

use base64::{prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD}, Engine};
use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use rand::{rngs::ThreadRng, Rng};
use sha1::{Digest, Sha1};

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
    pub fn mint(bits: u32, date_width: u8, resource: &String) -> Token {
        let mut rand = [0 as u8; 12];
        rand::rng().fill(&mut rand);

        let mut stamp = Token {
            ver: 1,
            bits,
            date: current_naive_date_time(),
            date_width,
            resource: resource.clone(),
            ext: "".to_string(),
            rand: rand.to_vec(),
            counter: 0,
        };

        loop {
            if stamp.check() {
                return stamp.clone();
            }

            stamp.counter += 1;
        }
    }

    pub fn check(&self) -> bool {
        let expires_after = Duration::days(2);
        let delta = current_naive_date_time() - self.date;

        if delta >= expires_after {
            return false;
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
            return false;
        }

        return true;
    }

    pub fn to_string(&self) -> String {
        let date_format = match self.date_width {
            10 => "%y%m%d%H%M".to_string(),
            12 => "%y%m%d%H%M%S".to_string(),
            _ => "%y%m%d".to_string(),
        };

        let rand = BASE64_STANDARD_NO_PAD.encode(&self.rand);
        let counter = BASE64_STANDARD_NO_PAD.encode(self.counter.to_string());

        println!("attempts: {}", self.counter);

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
}

impl TryFrom<String> for Token {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() < 13 || value.split(":").count() != 7 {
            return Err(());
        }

        let mut iter = value.split(":");

        let ver = iter.next()
            .expect("Couldn't read 'ver' field")
            .parse::<u8>()
            .expect("Couldn't parse 'ver'");

        let bits = iter.next()
            .expect("Couldn't read 'bits' field")
            .parse::<u32>()
            .expect("Couldn't parse 'bits'");

        let date_str = iter.next()
            .expect("Couldn't read 'date' field");

        let date_width = date_str.len() as u8;

        let date = match date_width {
            6 => NaiveDateTime::new(
                NaiveDate::parse_from_str(date_str, "%y%m%d").expect("Couldn't parse 'date' with length 6"),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            ),
            10 => NaiveDateTime::parse_from_str(date_str, "%y%m%d%H%M").expect("Couldn't parse 'date' with length 10"),
            12 => NaiveDateTime::parse_from_str(date_str, "%y%m%d%H%M%S").expect("Couldn't parse 'date' with length 12"),
            _ => return Err(()),
        };

        let resource = iter
            .next()
            .expect("Couldn't read 'resource' field")
            .to_string();

        let ext = iter
            .next()
            .expect("Couldn't read 'ext' field")
            .to_string();

        let rand = BASE64_STANDARD_NO_PAD
            .decode(
                iter
                    .next()
                    .expect("Couldn't read 'rand' field")
            )
            .expect("Couldn't decode 'rand'");

        let counter_str = iter
            .next()
            .expect("Couldn't read 'counter' field")
            .chars()
            .skip_while(|x| *x == '0')
            .collect::<String>();

        let counter_not_padded = BASE64_STANDARD_NO_PAD
            .decode(counter_str)
            .expect("Couldn't decode 'counter'");

        let mut counter_arr = [0 as u8; 8];
        let counter_base = (8 - counter_not_padded.len() - 1) as usize;

        for i in 0..counter_not_padded.len() {
            counter_arr[counter_base + i] = *(counter_not_padded.iter().nth(i).unwrap());
        }

        let counter = counter_arr.as_slice().read_u64::<BigEndian>().unwrap();

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

fn current_naive_date_time() -> NaiveDateTime {
    return DateTime::parse_from_rfc3339(Local::now().to_rfc3339().as_str())
        .unwrap()
        .naive_local();
}

fn random_alphabet_vec(rng: &mut ThreadRng, size: usize, alphabet: &str) -> Vec<u8> {
    return [0..size]
        .iter()
        .map(|_| u8::try_from(
            alphabet
                .chars()
                .nth(rng.random_range(0..alphabet.len()))
                .unwrap())
        .unwrap())
        .collect::<Vec<u8>>();
}

#[cfg(test)]
mod test;

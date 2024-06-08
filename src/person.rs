use chrono::Datelike;

use crate::error::*;

pub static OLDEST_PERSON: u8 = 120;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub pin: String,
    pub age: Result<u8, PersonError>,
}

#[derive(Debug)]
struct Birthday {
    year: u16,
    month: u8,
    day: u8,
}

impl Person {
    /// Skapa en ny person från namn och personnummer.
    pub fn new(name: &str, pin: &str) -> Person {
        let name = name.to_string();
        let pin = pin.to_string();

        let birthday = Birthday::from_str(&pin);

        let age: Result<u8, PersonError> = match &birthday {
            Ok(b) => b.age(),
            Err(e) => Err(e.clone()),
        };

        Person { name, pin, age }
    }
}

impl Birthday {
    /// Skapa en ny födelsedag från år, månad och dag.
    fn new(year: u16, month: u8, day: u8) -> Birthday {
        Birthday { year, month, day }
    }

    /// Skapa en ny födelsedag från personnummer.
    fn from_str(s: &str) -> Result<Birthday, PersonError> {
        let pin = format!("{:>12}", s);
        let len = s.len();

        let year = match len {
            10 => {
                pin[2..4]
                    .parse::<u16>()
                    .map_err(|_| PersonError::InvalidPin("Couldnt parse pin".to_string()))?
                    + 2000
            }
            12 => pin[0..4]
                .parse::<u16>()
                .map_err(|_| PersonError::InvalidPin("Couldnt parse pin".to_string()))?,
            x => return Err(PersonError::InvalidPinLength(x)),
        };

        let month = pin[4..6]
            .parse::<u8>()
            .map_err(|_| PersonError::InvalidPin("Couldnt parse pin".to_string()))?;
        let day = pin[6..8]
            .parse::<u8>()
            .map_err(|_| PersonError::InvalidPin("Couldnt parse pin".to_string()))?;

        let birthday = Birthday::new(year, month, day);

        Ok(birthday)
    }

    fn age(&self) -> Result<u8, PersonError> {
        let current_date = chrono::Local::now().naive_local().date();

        let c_year = current_date.year() as u16;
        let c_month = current_date.month() as u8;
        let c_day = current_date.day() as u8;

        // If the difference in years is negative, the person is not born yet, so an error is
        // returned.
        let mut dy = c_year
            .checked_sub(self.year)
            .ok_or(PersonError::FutureBirthday)?;
        // If the current month is less than the birth month, or if the current month is the same
        // but the current day is less than the birth day, the person has not had their Birthday
        // yet, so the age is decreased by one.
        if c_month < self.month || (c_month == self.month && c_day < self.day) {
            dy -= 1;
        }

        // If the person is older than whats defined in OLDEST_PERSON, an error is returned.
        if dy > OLDEST_PERSON as u16 {
            return Err(PersonError::LongDead);
        }

        // The age can be u8 since its garantueed to be between 0 and OLDEST_PERSON, which is a u8.
        Ok(dy as u8)
    }
}

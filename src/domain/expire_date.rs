use std::error::Error;

use crate::{data::GenSerialData, input::get_user_input_custom};

pub struct ExpireDate {
    year: u16,
    month: u8,
    day: u8,
    name: String,
}

impl ExpireDate {
    pub fn new() -> Self {
        ExpireDate {
            name: "ExpireDate".to_owned(),
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

fn parse_to_date(s: &str) -> Result<(u16, u8, u8), Box<dyn Error>> {
    let (year, month, day): (u16, u8, u8) = (s[0..4].parse()?, s[4..6].parse()?, s[6..8].parse()?);
    if year < 2025 {
        return Err("The year must be 2025 or later.".into());
    }
    if !(1..=12).contains(&month) {
        return Err("The month must be between 1 and 12.".into());
    }
    if !(1..=31).contains(&day) {
        return Err("The day must be between 1 and 31.".into());
    }
    Ok((year, month, day))
}

impl GenSerialData for ExpireDate {
    fn get_input_from_user(&mut self) {
        println!("Please input the expiration date (YYYYMMDD) (e.g. 20251223): ",);
        let rawdata = get_user_input_custom(|s| {
            if s.len() != 8 {
                return Err("date must be 8 digits".into());
            }
            if !s.chars().all(|c| c.is_numeric()) {
                return Err("date must be numberic".into());
            }
            let _ = parse_to_date(s)?;
            Ok(s.to_owned())
        });
        self.put_rawdata(rawdata);
    }

    fn get_length(&mut self) -> usize {
        8
    }

    fn get_rawdata(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        let (year, month, day) = parse_to_date(&data).unwrap();
        self.year = year;
        self.month = month;
        self.day = day;
    }
}

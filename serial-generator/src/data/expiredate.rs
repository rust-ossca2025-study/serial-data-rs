use crate::traits::GenSerialData;
use crate::utils::get_user_input;
pub struct ExpireDate {
    year: u32,
    month: u32,
    day: u32,
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

impl GenSerialData for ExpireDate {
    fn get_input_from_user(&mut self) {
        println!("Please input the expiration date (YYYYMMDD) (e.g. 20250811) : ",);
        let rawdata = get_user_input();
        assert_eq!(rawdata.len(), 8); // 입력받은 데이터의 길이가 8인지 검증, YYYYMMDD에 /가 2개 들어가므로 8개임

        // 입력받은 날짜를 분리해서 year, month, day 필드에 저장
        // 동시에 year, month, day 필드에 저장된 값이 올바른지 검증
        self.year = rawdata[0..4].parse().unwrap();
        assert!(self.year >= 2021, "The year must be 2021 or later.");
        self.month = rawdata[4..6].parse().unwrap();
        assert!(
            self.month >= 1 && self.month <= 12,
            "The month must be between 1 and 12."
        );
        self.day = rawdata[6..8].parse().unwrap();
        assert!(
            self.day >= 1 && self.day <= 31,
            "The day must be between 1 and 31."
        );
    }

    fn verify(&mut self, data: &str) -> bool {
        let year = data[0..4].parse().unwrap();
        let month = data[4..6].parse().unwrap();
        let day = data[6..8].parse().unwrap();

        self.year == year && self.month == month && self.day == day
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

    fn put_rawdata(&mut self, _data: String) {
        unimplemented!()
    }
}

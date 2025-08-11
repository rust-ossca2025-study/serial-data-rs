use crate::traits::GenSerialData;
pub struct ProductID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl ProductID {
    pub fn new(digit: usize) -> Self {
        Self {
            name: "ProductID".to_owned(),
            digit,
            id: None,
        }
    }
}

impl GenSerialData for ProductID {
    fn get_length(&mut self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        self.id.clone().unwrap()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        self.id = Some(data);
    }
}

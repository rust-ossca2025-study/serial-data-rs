use crate::io::get_input;

pub struct CustomerID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl CustomerID {
    pub fn new(digit: usize) -> Self {
        CustomerID {
            name: "UserID".to_owned(),
            digit,
            id: None,
        }
    }
}

pub struct ProductID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl ProductID {
    pub fn new(digit: usize) -> Self {
        ProductID {
            name: "ProductID".to_owned(),
            digit,
            id: None,
        }
    }
}

trait GenSerialData {
    fn get_input_from_user(&mut self) {
        println!(
            "Please input {}-digits for {}: ",
            self.get_length(),
            self.get_name(),
        );
        let input: String = get_input(|_| Ok(())).unwrap();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn verify(&mut self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
    }

    fn get_length(&mut self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, _data: String);
}

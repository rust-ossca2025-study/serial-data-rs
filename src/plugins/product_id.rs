use super::{GenSerialData, Plugin, get_user_input};

pub struct ProductID {
    id: Option<String>,
    digit: usize,
}

impl ProductID {
    pub fn new() -> Self {
        ProductID {
            id: None,
            digit: 8,
        }
    }

}

impl GenSerialData for ProductID {
    fn get_input_from_user(&mut self) {
        let prompt = format!("Please input {}-digits {}: ", self.digit, self.get_name());
        let input = get_user_input(&prompt);
        
        if self.verify(&input) {
            self.id = Some(input);
        } else {
            println!("Invalid input. Expected {} digits.", self.digit);
            self.get_input_from_user();
        }
    }

    fn verify(&self, data: &str) -> bool {
        data.len() == self.digit && data.chars().all(char::is_numeric)
    }

    fn get_rawdata(&self) -> String {
        self.id.clone().unwrap_or_default()
    }

    fn get_name(&self) -> String {
        "Product ID".to_string()
    }

}

impl Plugin for ProductID {}
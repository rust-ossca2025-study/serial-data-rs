use super::{GenSerialData, Plugin, get_user_input};

pub struct CustomerID {
    id: Option<String>,
    digit: usize,
}

impl CustomerID {
    pub fn new() -> Self {
        CustomerID {
            id: None,
            digit: 4,
        }
    }

}

impl GenSerialData for CustomerID {
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
        "Customer ID".to_string()
    }

}

impl Plugin for CustomerID {}
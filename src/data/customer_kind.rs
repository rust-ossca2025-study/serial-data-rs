use crate::{data::GenSerialData, input::get_user_input};

#[derive(Debug)]
pub enum CustomerKind {
    Business,
    Student,
    Company,
}

impl From<CustomerKind> for usize {
    fn from(item: CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1,
            CustomerKind::Student => 2,
            CustomerKind::Company => 3,
        }
    }
}

impl From<&CustomerKind> for usize {
    fn from(item: &CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1,
            CustomerKind::Student => 2,
            CustomerKind::Company => 3,
        }
    }
}

pub struct CustomerType {
    pub customer_type: Option<CustomerKind>,
    pub digit: usize,
    pub name: String,
}

impl CustomerType {
    pub fn new() -> Self {
        CustomerType {
            name: "CustomerType".to_owned(),
            digit: 1,
            customer_type: None,
        }
    }
}

impl GenSerialData for CustomerType {
    fn get_input_from_user(&mut self) {
        print!("customer type: ");
        print!(
            "{}-{:?}, ",
            usize::from(CustomerKind::Business),
            CustomerKind::Business
        );
        print!(
            "{}-{:?}, ",
            usize::from(CustomerKind::Student),
            CustomerKind::Student
        );
        print!(
            "{}-{:?}",
            usize::from(CustomerKind::Company),
            CustomerKind::Company
        );
        print!(
            "\nPlease input {}-digits for {}: ",
            self.get_length(),
            &self.get_name()
        );
        let input = get_user_input(&self.get_name(), self.get_length());
        self.put_rawdata(input);
    }

    fn get_length(&mut self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        if let Some(kind) = &self.customer_type {
            format!("{}", usize::from(kind))
        } else {
            "0".to_owned()
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        let kind = match data.as_str() {
            "1" => CustomerKind::Business,
            "2" => CustomerKind::Student,
            "3" => CustomerKind::Company,
            _ => CustomerKind::Business,
        };
        self.customer_type = Some(kind);
    }
}

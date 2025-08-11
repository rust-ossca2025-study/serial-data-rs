use crate::traits::GenSerialData;
use crate::utils::get_user_input;
#[derive(Clone, Debug)]

enum CustomerKind {
    Business,
    Student,
    Company,
}

impl From<&CustomerKind> for usize {
    fn from(item: &CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1, // 개인이 구매해서 사용하는 경우
            CustomerKind::Student => 2,  // 학생이 무료버전을 사용하는 경우
            CustomerKind::Company => 3,  // 회사에서 단체 구매한 경우
        }
    }
}

pub struct CustomerType {
    customer_type: Option<CustomerKind>,
    digit: usize,
    name: String,
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
        let input: String;

        println!("Please input customer type: ");
        print!(
            "{}-{:?}, ",
            usize::from(&CustomerKind::Business),
            CustomerKind::Business
        );
        print!(
            "{}-{:?}, ",
            usize::from(&CustomerKind::Student),
            CustomerKind::Student
        );
        print!(
            "{}-{:?}",
            usize::from(&CustomerKind::Company),
            CustomerKind::Company
        );
        input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn get_length(&mut self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        if let Some(kind) = self.customer_type.as_ref() {
            return format!("{}", usize::from(kind));
        } else {
            return "0".to_owned();
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

use crate::utils::get_user_input;

pub trait GenSerialData {
    fn get_input_from_user(&mut self) {
        let input: String;

        println!(
            "Please input {}-digits for {}: ",
            self.get_length(),
            self.get_name()
        );
        input = get_user_input();
        assert_eq!(
            input.len(),
            self.get_length(),
            "입력 길이가 {}와 일치하지 않습니다.",
            self.get_length()
        );
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

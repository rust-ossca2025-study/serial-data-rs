use crate::input::get_user_input;

pub trait GenSerialData {
    fn get_input_from_user(&mut self) {
        print!(
            "Please input {}-digits for {}: ",
            self.get_length(),
            &self.get_name()
        );
        let input = get_user_input(&self.get_name(), self.get_length());
        self.put_rawdata(input);
    }

    fn get_length(&mut self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, _data: String);
}

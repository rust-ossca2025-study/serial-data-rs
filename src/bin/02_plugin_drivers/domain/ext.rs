use super::traits::GenSerialData;

pub trait GenSerialDataVecExt {
    fn collect_data(&mut self);
    fn generate_serial(&mut self) -> String;
    fn validate_serialized_data(&mut self, serialized_data: &str) -> bool;
}

impl GenSerialDataVecExt for Vec<Box<dyn GenSerialData>> {
    fn collect_data(&mut self) {
        for item in self.iter_mut() {
            item.get_input_from_user();
        }
    }

    fn generate_serial(&mut self) -> String {
        let mut data = String::new();
        for item in self.iter_mut() {
            data.push_str(&item.get_rawdata());
        }
        data
    }

    fn validate_serialized_data(&mut self, serialized_data: &str) -> bool {
        let mut offset = 0;
        for item in self.iter_mut() {
            let len = item.get_length();
            let rawdata = &serialized_data[offset..offset + len];
            println!("Verify {}: {}", item.get_name(), rawdata);
            println!("Verify result: {}", item.verify(rawdata));
            offset += len;
        }
        true
    }
}

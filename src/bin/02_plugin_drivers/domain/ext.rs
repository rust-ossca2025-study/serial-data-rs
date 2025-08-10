use crate::domain::traits::GenSerialData;

pub trait GenSerialDataVecExt {
    fn collect_data(&mut self);
    fn generate_serial(&mut self) -> String;
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
}

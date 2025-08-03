use lazy_static::lazy_static;
use magic_crypt::{MagicCrypt256, MagicCryptTrait, new_magic_crypt};

const CRYPT_KEY: &str = "magickey";

lazy_static! {
    static ref MAGIC_CRYPT_BASE: MagicCrypt256 = new_magic_crypt!(CRYPT_KEY, 256);
}

pub const CUSTOMERID_LENGTH: usize = 4;
pub const PRODUCTID_LENGTH: usize = 8;
pub const TOTAL_ID_LENGTH: usize = CUSTOMERID_LENGTH + PRODUCTID_LENGTH;

#[derive(Clone, PartialEq, Debug)]
pub struct SerialData {
    pub customerid: String,
    pub productid: String,
}

impl SerialData {
    pub fn concat(&self) -> String {
        format!("{}{}", self.customerid, self.productid)
    }

    pub fn encrypt(&self) -> String {
        let plain_serial = self.concat();
        MAGIC_CRYPT_BASE.encrypt_str_to_base64(&plain_serial)
    }

    pub fn from_string(dec: String) -> SerialData {
        if dec.len() < TOTAL_ID_LENGTH {
            panic!(
                "Serial must be at least {TOTAL_ID_LENGTH} characters, got {}",
                dec.len()
            );
        }

        SerialData {
            customerid: dec[0..CUSTOMERID_LENGTH].to_owned(),
            productid: dec[CUSTOMERID_LENGTH..TOTAL_ID_LENGTH].to_owned(),
        }
    }

    pub fn print(&self) {
        println!("Customer ID: {}", self.customerid);
        println!("Product ID: {}", self.productid);
    }
}

pub fn decrypt_serialdata(encrypted_serial: String) -> String {
    MAGIC_CRYPT_BASE
        .decrypt_base64_to_string(encrypted_serial)
        .unwrap_or_else(|e| {
            eprintln!("Decryption failed: {e:?}");
            panic!("Invalid encrypted serial data provided: {e:?}");
        })
}

#[cfg(test)]
mod tests {
    use super::{SerialData, decrypt_serialdata};

    #[test]
    fn concat_serialdata() {
        let serialdata = SerialData {
            customerid: String::from("1234"),
            productid: String::from("qwerasdf"),
        };

        let expected = String::from("1234qwerasdf");

        assert_eq!(serialdata.concat(), expected);
    }

    #[test]
    fn get_serialdata_from_string() {
        let plain_string = String::from("1234qwerasdf");

        let expected = SerialData {
            customerid: String::from("1234"),
            productid: String::from("qwerasdf"),
        };

        assert_eq!(SerialData::from_string(plain_string), expected);
    }

    #[test]
    fn e2e_serialize() {
        let expected = SerialData {
            customerid: String::from("1234"),
            productid: String::from("qwerasdf"),
        };

        let input = expected.clone();
        let serial = input.encrypt();
        let decrypted = decrypt_serialdata(serial);

        let serialdata = SerialData::from_string(decrypted);

        assert_eq!(serialdata, expected);
    }
}

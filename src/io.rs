use std::error::Error;

type Validate = fn(&str) -> Result<(), Box<dyn Error>>;

pub fn get_input(validate: Validate) -> Result<String, Box<dyn Error>> {
    let buf = &mut String::new();
    std::io::stdin().read_line(buf)?;

    validate(buf.trim())?;

    Ok(buf.trim_end().to_string())
}

pub mod customer_id;
pub mod product_id;

use std::io::{stdin, stdout, Write};

pub trait GenSerialData {
    fn get_input_from_user(&mut self);
    fn verify(&self, data: &str) -> bool;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
}

pub trait Plugin: GenSerialData {}

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }

    pub fn get_plugins_mut(&mut self) -> &mut Vec<Box<dyn Plugin>> {
        &mut self.plugins
    }
}
mod plugins;
mod payload;
mod provider;
mod providers;
mod service;

use std::sync::Arc;
use plugins::customer_id::CustomerID;
use plugins::product_id::ProductID;
use plugins::PluginRegistry;
use payload::{InputPayload, StandardInput};
use providers::magic_crypt_provider::MagicCryptProvider;
use providers::sha256_provider::Sha256Provider;
use service::SerialNumberService;

fn collect_plugin_data() -> StandardInput {
    let mut registry = PluginRegistry::new();
    
    registry.register(Box::new(CustomerID::new()));
    registry.register(Box::new(ProductID::new()));
    
    println!("\n=== Collecting Serial Data ===");
    for plugin in registry.get_plugins_mut() {
        plugin.get_input_from_user();
    }
    
    StandardInput::from_plugins(registry.get_plugins())
}

fn display_menu() {
    println!("\n=== Serial Number Generator ===");
    println!("1. Generate with MagicCrypt (AES-256)");
    println!("2. Generate with SHA-256");
    println!("3. Validate existing serial");
    println!("4. List available providers");
    println!("5. Exit");
    print!("Select option: ");
    use std::io::{stdout, Write};
    let _ = stdout().flush();
}

fn main() {
    let mut service = SerialNumberService::new();
    
    service.register(Arc::new(MagicCryptProvider::with_default_key()));
    service.register(Arc::new(Sha256Provider::with_default_salt()));
    
    loop {
        display_menu();
        
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        
        match choice.trim() {
            "1" => {
                let input = collect_plugin_data();
                match service.create_serial("MagicCrypt", &input) {
                    Ok(serial) => {
                        println!("\nGenerated Serial (MagicCrypt):");
                        println!("{}", serial);
                        
                        if let Ok(decrypted) = StandardInput::from_canonical_string(&input.to_canonical_string()) {
                            println!("Original data: {}", decrypted.to_canonical_string());
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                let input = collect_plugin_data();
                match service.create_serial("SHA256", &input) {
                    Ok(serial) => {
                        println!("\nGenerated Serial (SHA-256):");
                        println!("{}", serial);
                        println!("Original data: {}", input.to_canonical_string());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                print!("Enter provider name (MagicCrypt/SHA256): ");
                use std::io::{stdout, Write};
                let _ = stdout().flush();
                
                let mut provider = String::new();
                std::io::stdin()
                    .read_line(&mut provider)
                    .expect("Failed to read input");
                
                print!("Enter serial to validate: ");
                let _ = stdout().flush();
                
                let mut serial = String::new();
                std::io::stdin()
                    .read_line(&mut serial)
                    .expect("Failed to read input");
                
                match service.check_validity(provider.trim(), serial.trim()) {
                    Ok(valid) => {
                        if valid {
                            println!("Serial is valid!");
                        } else {
                            println!("Serial is invalid!");
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                println!("\nAvailable Providers:");
                for (name, description) in service.list_providers() {
                    println!("  - {}: {}", name, description);
                }
            }
            "5" => {
                println!("END");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
use std::fs;  
use std::env;  
use sha3::{Digest, Sha3_512};  

fn main() {  
    let args: Vec<String> = env::args().collect();  
    if args.len() != 3 {  
        eprintln!("Usage: sbom-verifier <sbom_file.json> <expected_hash>");  
        eprintln!("Example: sbom-verifier my-app.sbom a1b2c3d4...");  
        std::process::exit(1);  
    }  

    let sbom_path = &args[1];  
    let expected_hash = &args[2];  

    let sbom_content = match fs::read(sbom_path) {  
        Ok(content) => content,  
        Err(e) => {  
            eprintln!("Error reading file '{}': {}", sbom_path, e);  
            std::process::exit(1);  
        }  
    };  

    let mut hasher = Sha3_512::new();  
    hasher.update(&sbom_content);  
    let actual_hash = hex::encode(hasher.finalize());  

    if actual_hash == *expected_hash {  
        println!("VERIFIED");  
    } else {  
        println!("POISONED");  
        println!("Expected: {}", expected_hash);  
        println!("Actual:   {}", actual_hash);  
    }  
}  

// external dependencies listed in Cargo.toml
// https://crates.io/crates/md5 
// https://crates.io/crates/rand

// compile with [cargo run] run with ./main

// use md5::compute;
use rand::Rng; // to bring Rng trait 
use std::fs::File;
use std::io::Write;

fn generate_preimage(length: usize) -> Vec<u8> {
    let mut rng = rand::rng(); // mut for mutable
    let mut preimage = Vec::new();

    for _ in 0..length { // if i unused warning
        preimage.push(rng.random_range(0..=255));
    }

    preimage // return statement, last statement w/o semicolon
}

fn comp_md5(preimage: &[u8]) -> [u8; 16] {
    md5::compute(preimage).0 // return tuple extract
}

fn pin(filename: &str, preimage: &[u8], hash: &[u8; 16]) -> std::io::Result<()> {

    let mut file = File::create(filename)?;
    writeln!(file, "(set_default_modulus 52435875175126190479447740508185965837690552500527637822603658699938581184513")?;
    writeln!(file, "    (let (")?;
    for (i, byte) in preimage.iter().enumerate() {
        writeln!(file, "        (pre_image.{} #x{:02x})", i, byte)?;
    }

    for (i, byte) in hash.iter().enumerate(){
        writeln!(file, "        (expected.{} #x{:02x})", i, byte)?;
    }

    writeln!(file, "    ) false)")?;
    writeln!(file, ")")?;

    Ok(())
}

fn vin(filename: &str, hash: &[u8; 16]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "(set_default_modulus 52435875175126190479447740508185965837690552500527637822603658699938581184513")?;
    writeln!(file, "    (let (")?;
    
    for (i, byte) in hash.iter().enumerate() {
        writeln!(file, "        (expected.{} #x{:02x})", i, byte)?;
    }
    
    writeln!(file, "    ) false)")?;
    writeln!(file, ")")?;
    
    Ok(())
}

fn main() {
    let preimage_length = 5;
    let num_tests = 5;
    
    println!("Generating {} test cases with preimage length {}...\n", 
             num_tests, preimage_length);
    
    for test_num in 0..num_tests {
        let preimage = generate_preimage(preimage_length);
        let hash = comp_md5(&preimage);
        
        let pin_filename = format!("test_{}.zok.pin", test_num);
        let vin_filename = format!("test_{}.zok.vin", test_num);
        
        match pin(&pin_filename, &preimage, &hash) {
            Ok(_) => println!("✓ Created {}", pin_filename),
            Err(e) => println!("✗ Error creating {}: {}", pin_filename, e),
        }
        
        match vin(&vin_filename, &hash) {
            Ok(_) => println!("✓ Created {}", vin_filename),
            Err(e) => println!("✗ Error creating {}: {}", vin_filename, e),
        }
        
        println!("  Preimage: {:?}", preimage);
        print!("  MD5 Hash: ");
        for byte in hash.iter() {
            print!("{:02x}", byte);
        }
        println!("\n");
    }
    
    println!("Done! Generated {} test case pairs.", num_tests);
}
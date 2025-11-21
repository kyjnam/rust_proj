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

    let mut file = File::create(filename)?; // 
    writeln!(file, "(set_default_modulus 52435875175126190479447740508185965837690552500527637822603658699938581184513")?;
    writeln!(file, "    (let (")?;

    for i in 0..preimage.len() {
        writeln!(file, "        (pre_image.{} #x{:02x})", i, preimage[i])?;
    }
    
    for i in 0..hash.len() {
        writeln!(file, "        (expected.{} #x{:02x})", i, hash[i])?;
    }
    writeln!(file, "    ) false)")?;
    writeln!(file, ")")?;

    Ok(()) // io result, nothing to return
}

fn vin(filename: &str, hash: &[u8; 16]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "(set_default_modulus 52435875175126190479447740508185965837690552500527637822603658699938581184513")?;
    writeln!(file, "    (let (")?;
    for (i, byte) in hash.iter().enumerate() {
        writeln!(file, "        (expected.{} #x{:02x})", i, byte)?; // hexadecimal, padded 2 digits
    }
    writeln!(file, "    ) false)")?;
    writeln!(file, ")")?;
    
    Ok(())
}

fn main() {
    let preimage_length = 5;
    let num_tests = 5;
    
    for test_num in 0..num_tests {
        let preimage = generate_preimage(preimage_length);
        let hash = comp_md5(&preimage);
        
        let pin_filename = format!("test{}.zok.pin", test_num);
        let vin_filename = format!("test{}.zok.vin", test_num);

        let _ = pin(&pin_filename, &preimage, &hash);
        let _ = vin(&vin_filename, &hash);
    }
}
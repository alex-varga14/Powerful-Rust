use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
}; 
use sha256::{
    digest_bytes,
    digest,
};

const SHA256_HEX_STRING_LENGTH: usize = 64;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha256_cracker: <wordlist.txt> <sha256_hash>"); //Reminder: println with ! is not a function,
        return Ok(());                                              // it is a MACRO
    }

    let hash_to_crack = args[2].trim();
    //println!("Len: {}", hash_to_crack.len());
    if hash_to_crack.len() != SHA256_HEX_STRING_LENGTH {
        return Err("sha256 hash is not valid!".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack ==
            sha256::digest(common_password) {
                println!("Password found: {}", &common_password);
                return Ok(());
            }
    }
    println!("Password not found in wordlist.");

    Ok(())
}

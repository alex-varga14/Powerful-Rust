/*
  SHA-1 is hash function used by a lot of old websites to store the passwords of the users.

    -Hashed passwords cannot be recovered from its hash
    -By storing the users hash in their database, a website can assert that a given user has the knowledge of its
     password without storing the password in cleartext or plaintext, by comparing hashes.
    -So if the websites database is breached, there is no way to recover the password and access the users data.

    -In REALITY, imagine weve breached a website and want to recover the credentials of the users
     to gain access to their accounts
    -This is where a "hash cracker" comes in handy
        Program will try many hashes in order to crack it
*/
// imports env from std lib
// We will "Box" errors: we will allow program to return any error type that implements the error trait
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
}; 
use sha1::Digest;

// Take too much time to test all possible combinations of letters, numbers and special chars,
// we need to reduce the number of SHA-1 hashes generated
// For this we will use a special kind of directory called a wordlist, which contains the most common passwords

// To actually compute the SHA-1 hashes, we will use a crate

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // env::args() calls args() func from module and returns an iterator which can be collected

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>"); //Reminder: println with ! is not a function,
        return Ok(());                                              // it is a MACRO
        // MACRO has the advantage of being compile-time evaluated and checked and thus prevents
        // vulnerabilities such as "format string vulnerabilities"
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid!".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        //println!("encode: {}",  &hex::encode(sha1::Sha1::digest(common_password.as_bytes())));
        if hash_to_crack ==
            &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
                println!("Password found: {}", &common_password);
                return Ok(());
            }
    }

    println!("Password not found in wordlist.");

    Ok(())
}

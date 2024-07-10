use rayon::{iter::ParallelIterator, str::ParallelString};

pub struct Rot13Encryption;

impl Rot13Encryption {
    pub fn encrypt<'a>(string: &'a str) -> String {
        string.bytes().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = {
                    if c.is_ascii_lowercase() {
                        'a'
                    } else {
                        'A'
                    }
                } as u8;
                return (base + ((c as u8 - base + 13) % 26)) as char;
            }
            c as char
        }).collect::<String>()
    }

    pub fn decrypt<'a>(string: &'a str) -> String {
        Rot13Encryption::encrypt(string)
    }
}

pub struct Rot13EncryptionRayon;

impl Rot13EncryptionRayon {
    pub fn encrypt<'a>(string: &'a str) -> String {
        string.par_chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = {
                    if c.is_ascii_lowercase() {
                        'a'
                    } else {
                        'A'
                    }
                } as u8;
                return (base + ((c as u8 - base + 13) % 26)) as char; // add number and modular it then add it to base and turn it to char
                // u8 is awesome :3
            }
            c
        }).collect::<String>()
    }

    pub fn decrypt<'a>(string: &'a str) -> String {
        Rot13Encryption::encrypt(string)
    }
}
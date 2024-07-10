use std::{io::Write, sync::{Arc, Mutex}};

use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct Rot13Encryption;

impl Rot13Encryption {
    pub fn encrypt<'a>(string: &'a str) -> String {
        let mut encrypted = String::new();
        for c in string.chars() {
            if ASCII_LOWER.contains(&c.to_ascii_lowercase()) {
                match ASCII_LOWER.iter().position(|a| c.eq_ignore_ascii_case(a)) {
                    Some(mut i) => {
                        i += 1; // index quirks
                        if i + 13 > 26 {
                            i = (i + 13) - 26
                        } else {
                            i += 13;
                        }
                        i -= 1; // index quirks
                        let res = {
                            if c.is_uppercase() {
                                &ASCII_LOWER[i].to_string().to_uppercase()
                            } else {
                                &ASCII_LOWER[i].to_string()
                            }
                        };
                        encrypted += res;
                        continue;
                    }
                    None => unreachable!("How"),
                }
            }
            encrypted += &c.to_string();
        }
        encrypted
    }

    pub fn decrypt<'a>(string: &'a str) -> String {
        Rot13Encryption::encrypt(string)
    }
}

pub struct Rot13EncryptionRayon;

impl Rot13EncryptionRayon {
    pub fn encrypt<'a>(string: &'a str) -> String {
        let encrypted = std::sync::Arc::new(Mutex::new(String::with_capacity(string.len())));
        string
            .par_chars()
            .for_each_with(Arc::clone(&encrypted), |s, c| {
                if ASCII_LOWER.contains(&c.to_ascii_lowercase()) {
                    match ASCII_LOWER
                        .par_iter()
                        .position_any(|a| c.eq_ignore_ascii_case(a))
                    {
                        Some(mut i) => {
                            i += 1;
                            if i + 13 > 26 {
                                i = (i + 13) - 26
                            } else {
                                i += 13;
                            }
                            i -= 1; // index quirks
                            let res = {
                                if c.is_uppercase() {
                                    ASCII_LOWER[i].to_ascii_uppercase()
                                } else {
                                    ASCII_LOWER[i]
                                }
                            };
                            let mut v = s.lock().expect("Failed to lock");
                            let index = string.par_chars().collect::<Vec<char>>().par_iter().position_any(|a| c.eq_ignore_ascii_case(a)).unwrap();
                            (*v).insert(index, res);
                            return;
                        }
                        None => unreachable!("WHAT"),
                    }
                }
                let mut v = s.lock().expect("Failed to lock");
                *v += &c.to_string();
            });
        let locked = encrypted.lock().unwrap();
        locked.to_string()
    }

    pub fn decrypt<'a>(string: &'a str) -> String {
        Rot13Encryption::encrypt(string)
    }
}

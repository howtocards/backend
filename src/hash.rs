use sha2::{Digest, Sha256};
use std::str::from_utf8;
use std::fmt::Display;

pub fn hash_string<S: AsRef<[u8]>>(value: S) -> String {
    let mut hasher = Sha256::default();

    hasher.input(value.as_ref());

    hasher.result().iter().map(|b| format!("{:02X}", b)).collect()
}

pub fn hash_password<P, S>(password: P, salt: S) -> String
where
    P: Display,
    S: Display,
{
    hash_string(format!("{}${}", password, salt))
}

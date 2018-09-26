//! Hashing utilites

use sha2::{Digest, Sha256};
use std::{fmt::Display, str::from_utf8};

/// Hash string with sha256
///
/// # Examples
///
/// ```
/// # extern crate howtocards;
/// use howtocards::hasher::hash_string;
///
/// assert_eq!(hash_string("Foo"), "1CBEC737F863E4922CEE63CC2EBBFAAFCD1CFF8B790D8CFD2E6A5D550B648AFA".to_string());
/// ```
pub fn hash_string<S: AsRef<[u8]>>(value: S) -> String {
    let mut hasher = Sha256::default();

    hasher.input(value.as_ref());

    hasher
        .result()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect()
}

/// Hash password with custom salt
pub fn hash_password<P, S>(password: P, salt: S) -> String
where
    P: Display,
    S: Display,
{
    hash_string(format!("{}${}", password, salt))
}

/// Check hashed password with salt and original password
pub fn validate_password<P, S>(hash: &String, salt: S, password: P) -> bool
where
    P: Display,
    S: Display,
{
    hash_password(password, salt).eq(hash)
}

mod test {
    use super::*;

    #[test]
    fn hash_string_should_get_different_results_for_different_input() {
        assert_ne!(hash_string("Foo"), hash_string("Foo1"));
        assert_ne!(hash_string("Foo"), hash_string("foo"));
    }

    #[test]
    fn hash_string_should_get_equal_results_for_equal_input() {
        assert_eq!(hash_string("Foo"), hash_string("Foo"));
        assert_eq!(hash_string("Bar"), hash_string("Bar"));
    }

    #[test]
    fn hash_password_use_hash_string() {
        assert_eq!(hash_string("Foo$Bar"), hash_password("Foo", "Bar"));
    }
}

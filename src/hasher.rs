use sha2::{Digest, Sha256};
use std::fmt::Display;
use std::str::from_utf8;

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

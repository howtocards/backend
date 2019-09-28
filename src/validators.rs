use regex::Regex;

pub fn check_username<T: AsRef<str>>(username: T) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[A-Za-z0-9_][A-Za-z0-9_]+(?:[ \-\._][A-Za-z0-9]+)*$").expect("username regex");
    }

    RE.is_match(username.as_ref())
}

use regex::Regex;

const REGEX_EMAIL: &str = r"^([a-zA-Z0-9_+]([a-zA-Z0-9_+.]*[a-zA-Z0-9_+])?)@([a-zA-Z0-9]+([\-\.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,6})";

pub fn email(str: &str) -> bool {
    Regex::new(REGEX_EMAIL).unwrap().is_match(str)
}

#[cfg(test)]
mod lib_utilities_validate {

    use crate::validate::email;

    #[test]
    fn test() {
        assert_eq!(email(""), false);
        assert_eq!(email("2cgAEXAlNE@danavation.com"), true);
        assert_eq!(email("user@danavation.com"), true);
        assert_eq!(email("foo@bar.com"), true);
        assert_eq!(email("42@c.com"), true);
        assert_eq!(email("f@42.co"), true);
        assert_eq!(email("foo@4-2.team"), true);
        assert_eq!(email(".x@c.com"), false);
        assert_eq!(email("x.@c.com"), false);
        assert_eq!(email("foo bar@bar.com"), false);
        assert_eq!(email(" bar@bar.com"), false);
        assert_eq!(email("foo_bar@bar.com"), true);
        assert_eq!(email("_bar@bar.com"), true);
        assert_eq!(email("foo_@bar.com"), true);
        assert_eq!(email("foo+bar@bar.com"), true);
        assert_eq!(email("+bar@bar.com"), true);
        assert_eq!(email("foo+@bar.com"), true);
        assert_eq!(email("foo.lastname@bar.com"), true);
    }
}
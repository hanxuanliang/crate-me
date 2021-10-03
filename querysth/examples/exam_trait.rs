use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<T> Parse for T where T: Defult + FromStr {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)").unwrap();
        let d = || Default::default();
        if let Some(cap) = re.captures(s) {
            cap
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            d()
        }
    }
}

#[test]
fn parse_works() {
    assert_eq!(u8::parse("123dadad"), 123);
    assert_eq!(u8::parse("123.123adbd"), 123);
    assert_eq!(u8::parse("adnd"), 0);
}

fn main() {
    println!("{}", u8::parse("123dadad"));
}

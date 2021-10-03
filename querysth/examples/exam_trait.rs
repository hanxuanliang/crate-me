use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(cap) = re.captures(s) {
            cap.get(0).map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
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

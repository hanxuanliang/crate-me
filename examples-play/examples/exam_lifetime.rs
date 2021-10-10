fn strtok<'a>(s: &mut &'a str, d: char) -> &'a str {
    if let Some(i) = s.find(d) {
        let prefix = &s[..i];
        let suffix = &s[(i + d.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_string();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is {}, s1 is {}, s is {}", hello, s1, s)
}

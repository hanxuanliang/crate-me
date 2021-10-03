use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error> where Self: Sized;
}

/// Default 返回数据结构缺省值
/// FromStr https://runebook.dev/zh-CN/docs/rust/std/str/trait.fromstr
/// 本例中会调用 str::parse 返回的类型为：FromStr trait；所以需要加一个泛型参数约束，应对不同的类型
impl<T> Parse for T where T: Default + FromStr {
    /// rust关联类型：https://rustcc.cn/article?id=1a2e348e-a4d0-4ee1-9368-353730f2e212
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        // 会按照预设的匹配规则 -> 如果没有匹配上会直接走 〈else 分支〉
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(cap) = re.captures(s) {
            cap
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

#[test]
fn parse_works() {
    assert_eq!(u8::parse("123dadad"), Ok(123));
    assert_eq!(f32::parse("123.123adbd"), Ok(123.123));
    assert_eq!(u8::parse("adnd"), Err("failed to parse string".into()));
}

fn main() {
    println!("{:?}", u8::parse("123dadad"));
}

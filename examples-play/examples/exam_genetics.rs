fn print<T: ToString>(v :T) {
    println!("{}", v.to_string());
}

fn print_inline(v: impl ToString) {
    println!("{}", v.to_string());
}

fn main() {
    let c = 'a';
    let str_a = "a_yxc";
    
    // char 和 &str 都实现了 ToString trait
    print::<char>(c);
    print::<&str>(str_a);

    // 同样是静态分发：单态化的体现
    print_inline(c);
}

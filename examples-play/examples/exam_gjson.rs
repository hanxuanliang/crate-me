fn main() {
    let json_str = r#"{
        "name": {"first": "Tom", "last": "Anderson"},
        "age":37,
        "children": ["Sara","Alex","Jack"],
        "fav.movie": "Deer Hunter",
        "friends": [
          {"first": "Dale", "last": "Murphy", "age": 44, "nets": ["ig", "fb", "tw"]},
          {"first": "Roger", "last": "Craig", "age": 68, "nets": ["fb", "tw"]},
          {"first": "Jane", "last": "Murphy", "age": 47, "nets": ["ig", "tw"]}
        ]
      }"#;

    // .. 是针对 JSONlines 的语法标记

    let value = gjson::get(json_str, "friends.#.first");
    println!("{}", value);
}

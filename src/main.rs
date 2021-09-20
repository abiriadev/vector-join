pub fn join(vec: &Vec<&str>, separator: &str) -> String {
    let mut save = String::new();

    let mut c = 0;

    for str in vec {
        save.push_str(str);

        c += 1;

        if c != vec.len() {
            save.push_str(separator);
        }
    }

    save
}

fn main() {
    let mut v = vec!["apple", "banana", "orange"];

    println!("{}", join(&v, ";"));

    v.clear();

    println!("{}", join(&v, ";"));

    v.push("a");

    println!("{}", join(&v, ";"));

    v.push("b");

    println!("{}", join(&v, ";"));

    v.push("한글로 된 문자열");

    println!("{}", join(&v, "한글로 된 긴 길이의 구분자"));
}

const EL: [char; 18] = ['_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!'];

pub fn escape(s: &str) -> String {
    let mut new_str = String::new();
    for c in s.chars() {
        if EL.contains(&c) {
            new_str += r"\";
        }
        new_str.push(c);
    }
    println!("{}", new_str);
    new_str
}
const EL: [char; 18] = [
    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
];

pub fn escape(s: &str) -> String {
    let mut new_str = String::new();
    let mut start = 0;
    for (i, c) in s.bytes().enumerate() {
        if EL.contains(&(c as char)) {
            new_str += &s[start..i];
            new_str += "\\";
            new_str.push(c as char);
            start = i + 1;
        }
    }
    new_str.push_str(&s[start..]);
    new_str
}

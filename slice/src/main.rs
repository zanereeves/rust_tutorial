fn main() {
    let stringy = String::from("Hello Vanier");

    let word : usize = first_word_idx(&stringy);

    let slice = &stringy[0..word];
    println!("{}", slice);

    let s = first_word_str(&stringy);
    println!("{}", s);
}


fn first_word_str(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

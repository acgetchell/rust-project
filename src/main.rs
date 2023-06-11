fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string_literal: &str = "hello world";
    let word: &str = first_word(my_string_literal);
    println!("{}", word);
}

#[cfg(test)]
mod tests {
    use crate::first_word;

    #[test]
    fn it_works() {
        let my_string = String::from("hello world");
        let word = first_word(&my_string);
        assert_eq!(word, "hello");
    }
}

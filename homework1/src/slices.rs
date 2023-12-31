fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
pub fn check() {
    let s = String::from("hello world");

    let word = first_word(&s);
    println!("the first word is: {}", word);
}

pub fn slices1() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
    println!("{}", word)
}

pub fn slice_mass() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    print!("{:?}", slice);
    assert_eq!(slice, &[2, 3]);
}
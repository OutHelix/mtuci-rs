use std::io;

fn main() {
    println!("Enter the text!");
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    
    let len = op(word.clone());
    println!("Len {word} is {len}");

}

fn op (word: String) -> u32 {
    (word.len() - 2).try_into().unwrap()
}

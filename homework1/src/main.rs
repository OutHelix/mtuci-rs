mod variables;
mod functions;
mod ifs;
mod primitive_types;
fn main() {
    println!("ez domashka");
}

//-------------
//FIZZBUZZ TASK
//-------------

fn fizzbuzz() {
    for i in 1..=100 {
        match (i % 3, i % 5, i % 15) {
            (_, _, 0) => println!("FizzBuzz"),
            (_, 0, _) => println!("Buzz"),
            (0, _, _) => println!("Fizz"),
            _ => println!("{}", i),
        }
    }
}

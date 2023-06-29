pub fn first_func() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

//return func

fn five() -> i32 {
    5
}

pub fn return_func() {
    let x = five();

    println!("The value of x is: {x}");
}
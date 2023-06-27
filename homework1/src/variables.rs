pub fn variables1 () {
    println!("VARIABLES - 1\n--------");
    // **WRONG**

    // x = 5;
    // println!("x has the value {}", x);

    // **CORRECT**

    let x: i32 = 5;
    println!("x has the value {}", x);
}

pub fn variables2 () {
    println!("VARIABLES - 2\n--------");
    // **WRONG**

    // let x;
    // if x == 10 {
    //     println!("x is ten!");
    // } else {
    //     println!("x is not ten!");
    // }

    // **CORRECT**

    let x: i32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

pub fn variables3 () {
    println!("VARIABLES - 3\n--------");
    // **WRONG**

    // let x: i32;
    // println!("Number {}", x);

    // **CORRECT**

    let x: i32 = 5;
    println!("Number {}", x);
}

pub fn variables4 () {
    println!("VARIABLES - 4\n--------");
    // **WRONG**

    // let x = 3;
    // println!("Number {}", x);
    // x = 5; // don't change this line
    // println!("Number {}", x);

    // **CORRECT**

    let mut x: i32 = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

pub fn variables5 () {
    println!("VARIABLES - 5\n--------");
    // **WRONG**

    // let number = "T-H-R-E-E"; // don't change this line
    // println!("Spell a Number : {}", number);
    // number = 3; // don't rename this variable
    // println!("Number plus two is : {}", number + 2);

    // **CORRECT**

    let number: &str = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number: i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}

pub fn variables6 () {
    println!("VARIABLES - 6\n--------");
    // **WRONG**

    // const NUMBER = 3;
    // println!("Number {}", NUMBER);

    // **CORRECT**
    
    const NUMBER: i32 = 3;
    println!("Number {}", NUMBER);
}

pub fn functions1() {
    // **WRONG**

    // fn main() {
    //     call_me();
    // }

    // **CORRECT**

    fn call_me() {
        println!("Hello world!") //some text to check 
    }
    call_me();
}

pub fn functions2() {
    // **WRONG**

    // fn main() {
    //     call_me(3);
    // }
    
    // fn call_me(num:) {
    //     for i in 0..num {
    //         println!("Ring! Call number {}", i + 1);
    //     }
    // }

    // **CORRECT**
    
    call_me(3);
    
    fn call_me(num: i32) {
        for i in 0..num {
            println!("Ring! Call number {}", i + 1);
        }
    }
}

pub fn functions3() {
    // **WRONG**

    // fn main() {
    //     call_me();
    // }
    
    // fn call_me(num: u32) {
    //     for i in 0..num {
    //         println!("Ring! Call number {}", i + 1);
    //     }
    // }

    // **CORRECT**

    let num: u32 = 5;
    call_me(num);

    fn call_me(num: u32) {
        for i in 0..num {
            println!("Ring! Call number {}", i + 1);
        }
    }
}

pub fn functions4() {
    // **WRONG**

    // fn main() {
    //     let original_price = 51;
    //     println!("Your sale price is {}", sale_price(original_price));
    // }
    
    // fn sale_price(price: i32) -> {
    //     if is_even(price) {
    //         price - 10
    //     } else {
    //         price - 3
    //     }
    // }
    
    // fn is_even(num: i32) -> bool {
    //     num % 2 == 0
    // }

    // **CORRECT**

    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
    
    
    fn sale_price(price: i32) -> i32 {
        if is_even(price) {
            price - 10
        } else {
            price - 3
        }
    }
    
    fn is_even(num: i32) -> bool {
        num % 2 == 0
    }
}

pub fn functions5() {
    // **WRONG**

    // fn main() {
    //     let answer = square(3);
    //     println!("The square of 3 is {}", answer);
    // }
    
    // fn square(num: i32) -> i32 {
    //     num * num;
    // }

    // **CORRECT**
    
    let answer: i32 = square(3);
    println!("The square of 3 is {}", answer);

    
    fn square(num: i32) -> i32 {
        num * num
    }
}

pub fn primitive_types1() {
    // **WRONG**

    // let is_morning = true;
    // if is_morning {
    //     println!("Good morning!");
    // }

    // let // Finish the rest of this line like the example! Or make it be false!
    // if is_evening {
    //     println!("Good evening!");
    // }

    // **CORRECT**

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = true;
    if is_evening {
        println!("Good evening!");
    }
    
}

pub fn primitive_types2() {
    // **WRONG**

    // let my_first_initial = 'C';
    // if my_first_initial.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if my_first_initial.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    // let // Finish this line like the example! What's your favorite character?
    // // Try a letter, try a number, try a special character, try a character
    // // from a different language than your own, try an emoji!
    // if your_character.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if your_character.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    // **CORRECT**

    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character: char = '5';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

}

pub fn primitive_types3() {

    // Create an array with at least 100 elements in it where the ??? is.
    
    // **WRONG**

    // let a = ???

    // if a.len() >= 100 {
    //     println!("Wow, that's a big array!");
    // } else {
    //     println!("Meh, I eat arrays like that for breakfast.");
    // }

    // **CORRECT**

    let a = [5; 101];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}

#[test]
pub fn primitive_types4() {

    // Get a slice out of Array a where the ??? is so that the test passes.

    // **WRONG**

    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..=3];

    assert_eq!([2, 3, 4], nice_slice)

    // **CORRECT**
}

pub fn primitive_types5() {
    // Destructure the `cat` tuple so that the println will work.

    // **WRONG**

    // let cat = ("Furry McFurson", 3.5);
    // let /* your pattern here */ = cat;

    // println!("{} is {} years old.", name, age);

    // **CORRECT**

    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}

#[test]
pub fn primitive_types6() {
    // **WRONG**

    // Use a tuple index to access the second element of `numbers`. You can put the
    // expression for the second element where ??? is so that the test passes.

    // let numbers = (1, 2, 3);
    // // Replace below ??? with the tuple indexing syntax.
    // let second = ???;

    // assert_eq!(2, second,
    //     "This is not the 2nd number in the tuple!")

    // **CORRECT**

    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}

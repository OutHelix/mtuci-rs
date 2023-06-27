
// ------------------
// FIRST IF EXERCISE
// ------------------


pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables

    // OK
    
    if a < b {
        return b;
    } else {
        return a;
    }
}


// Don't mind this for now :)
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}


// ------------------
// SECOND IF EXERCISE
// ------------------

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz"{
        "bar"
    } else {
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
pub mod tests2 {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
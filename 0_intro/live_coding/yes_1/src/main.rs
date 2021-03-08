pub fn get_string() -> String {
    // Show that we can return it either using the return keyword or as an expression
    String::from("y")
}

// Either pass the value by reference or by moving it but note how it's different
pub fn show_string(s: &String) {
    println!("{}", s);
}

fn main() {
    // Ask if the variable needs to be mutable or not
    let s = get_string();

    // Make sure to show that we can use `loop` instead of `while true`
    loop {
        show_string(&s);
    }
}

// Explain conditional compilation
#[cfg(test)]
mod tests { // Quickly explain modules
    use super::*; // Explain `use` and refer to C++ namespaces + the asterisk

    // Refer to Java Attributes, like @JUnitTest that we did
    #[test]
    fn get_string_basic() {
        assert_eq!(get_string(), "y"); // Talk about assert_eq! and assert!
    }
}

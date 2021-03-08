//! # Objectives of the exercise
//!
//! The goal of this live coding session is to reimplement a simple binary (`yes`) in its
//! simplest form.
//!
//! ## Simplest simple form of yes
//!
//! When invoked without any arguments, `yes` simply prints the string "y" on stdout over
//! and over until the program is terminated.
//!
//! In order to do this, let's create a function `get_string()` that will return the correct
//! string to print. Then, let's implement a `show_string()` function to display it on
//! stdout. Obviously, this is much more complex than it needs to be. We could do all of
//! that in the main function. But we just learned about functions so let's use them!
//!
//! ## Unit testing yes
//!
//! We haven't yet seen the syntax for unit testing, so we'll explore it now. For now,
//! the tests should be really simple and basically make sure that `get_string()` returns
//! the correct string
//!
//! ## Getting command line arguments
//!
//! Calling `yes` with an argument will cause the program to print that argument over and
//! over instead of the string "y". In C, we would implement it by doing this:
//! ```c
//! int main(int argc, char **argv) {
//!     // 0 extra arguments provided - Remember, argc starts at 1
//!     if (argc < 2) {
//!         while (1) {
//!             printf("y\n");
//!         }
//!     } else {
//!         while (1) {
//!             printf("%s\n", argv[1]);
//!         }
//!     }
//! }
//! ```
//!
//! In Rust, you will need to use the `std::env::args()` function.
//!
//! The function `get_string()` will need to be modified. Your unit tests should reflect
//! that!
//!
//! ## Going further - Discovering crates and clap
//!
//! Using the clap crate, parse arguments given to your program and display something useful
//! when using the `-h|--help` option. The program should contain an author, a version
//! and a small description!
//!
//! ## Going further - Unit testing stdout

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

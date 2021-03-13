pub fn get_string(arg: Option<String>) -> String {
    // Remind that Option is either None or Some<T>
    // Introduce pattern matching
    match arg {
        None => String::from("y"),
        Some(value) => value,
    }

    // // You can give a nice example with numbers like
    // ```
    // let x = ...;
    //
    // match x {
    //      0 => String::from("zero"),
    //      1 | 3 | 5 => String::from("odd and less than 6"),
    //      _ => String::from("anything else"),
    // }
    // ```

    // // You can also show that EVERYTHING is an expression and you can do cool shit
    // // with `if let ...`
    // if let Some(argument) = arg {
    //     argument
    // } else {
    //     String::from("y")
    // }
}

pub fn show_string(s: &String) {
    println!("{}", s);
}

fn main() {
    // Explain that we could do the following
    // ```
    // use std::env;
    //
    // let args = env::args();
    // ```
    //
    // or
    //
    // ```
    // use std::env::args;
    //
    // let args = args();
    // ```
    //
    // Maybe talk a bit about `pub` functions versus private functions
    let mut args = std::env::args();

    // For this bit, first declare args as immutable: `let args = ...`
    // This way they see the error that pops when you try to call a method that takes
    // a mutable parameter with an immutable one
    let first_arg = args.nth(1);

    let s = get_string(first_arg);

    loop {
        show_string(&s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_string_basic() {
        assert_eq!(get_string(None), "y");
    }

    #[test]
    fn get_string_with_arg() {
        assert_eq!(get_string(Some(String::from("GISTRE"))), "GISTRE");
    }
}

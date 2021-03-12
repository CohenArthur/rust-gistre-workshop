// Show that you can use multiple modules at once
use clap::{App, Arg};

// Probably need to change this from Option<String> to Option<&str>
// since std::env::args().nth(...) returns a String but clap will return an &str.
// You can also get started on the differences between &str and String, and add an analogy
// to C/C++
pub fn get_string(arg: Option<&str>) -> String {
    match arg {
        None => String::from("y"),
        Some(value) => String::from(value),
    }
}

pub fn show_string(s: &String) {
    println!("{}", s);
}

fn main() {
    // Remind them that this is the Builder pattern
    let args = App::new("yes_but_in_Rust")
        .version("0.1.0")
        .author("Me, myself and I")
        .about("yes but in Rust!")
        .arg(Arg::with_name("input"))
        .get_matches();

    // Show that -h|--help is being generated automatically!

    let first_arg = args.value_of("input");

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
        assert_eq!(get_string(Some("GISTRE")), "GISTRE");
    }
}

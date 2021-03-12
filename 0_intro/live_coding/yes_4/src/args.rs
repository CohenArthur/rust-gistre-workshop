// modules should have a small case name - args in that case.
// You can show them how to create empty types and add functions/static methods to them
//
// ```
// pub struct Args;
//
// impl Args {
//     pub fn get_input() ...
//     pub fn get_string() ...
// }
// ```
//
// This is obviously the better way of doing this - and there are no warnings - but since
// we didn't show them how to create structs yet...

pub mod Args {
    use clap::{App, Arg};

    pub fn get_input() -> Option<String> {
        let args = App::new("yes_but_in_Rust")
            .version("0.1.0")
            .author("Me, myself and I")
            .about("yes but in Rust!")
            .arg(Arg::with_name("input"))
            .get_matches();

        args.value_of("input").map_or(None, |s| Some(s.to_string()))
    }

    // We can stay with an input: Option<String> but then pattern matching is harder.
    // However, it simplifies the code in the main function
    pub fn get_string(input: Option<&str>) -> String {
        match input {
            None => String::from("y"),

            // You can add fun stuff to show the power of pattern matching
            Some("monkey") => String::from("GISTRE"),
            Some("macaque") | Some("BLAIREAU") => String::from("Giga Elite"),

            // Show we can also use _ to say "everything that remains"
            Some(value) => String::from(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_string_basic() {
        assert_eq!(Args::get_string(None), "y");
    }

    #[test]
    fn get_string_with_arg() {
        assert_eq!(Args::get_string(Some("GISTRE")), "GISTRE");
    }
}

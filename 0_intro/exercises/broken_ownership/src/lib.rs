/// Create vector and fill it, then return it
/// Help me `modify_values` won't give my vector back :(
// FIXME
pub fn get_answer() -> Vec<i32> {
    let mut values = vec![0, 0];
    modify_values(values);
    values
}

// FIXME
fn modify_values(values: Vec<i32>) {
    values[0] = 4;
    values[1] = 2;
}

/// Print the first word of the "hello world" message, then free the message
/// Help me the borrowchecker is bullying me :(
// FIXME
pub fn print_hello() {
    let mut msg = String::from("hello world");
    let word = first_word(&msg); // Get a reference to the first word

    msg.clear(); // Empty the message
    println!("The first word is: {}", word); // Print first word
}

fn first_word(msg: &String) -> &str {
    msg.split_whitespace().nth(0).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_answer() {
        let values = get_answer();

        assert_eq!(values[0], 4);
        assert_eq!(values[1], 2);
    }

    #[test]
    fn test_hello() {
        print_hello();
    }
}

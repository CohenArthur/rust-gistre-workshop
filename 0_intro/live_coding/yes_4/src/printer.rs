pub mod Printer {
    fn show_string(s: &str) {
        println!("{}", s)
    }

    pub fn display(s: String) {
        loop {
            show_string(&s)
        }
    }
}

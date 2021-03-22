pub fn cat(path: &str) -> String {
    std::fs::read_to_string(path).expect("something went wrong reading the file")
}

fn main() {
    let res = cat("test");
    print!("{}", res)
}

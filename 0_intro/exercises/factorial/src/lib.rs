pub fn factorial(n: i32) -> i32 {
    let mut res = 1;
    for i in 1..=n {
        res = res * i;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facto_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn facto_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn facto_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn facto_3() {
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn facto_big() {
        assert_eq!(factorial(8), 40320);
    }
}

pub fn fibo(v: u32) -> u32 {
    if v == 0 {
        0
    }
    else {
        let mut f1 = 0;
        let mut f2 = 1;
        for _ in 1..v {
            let tmp = f2;
            f2 = f1 + f2;
            f1 = tmp;
        }
        f2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibo_0() {
        assert_eq!(0, fibo(0));
    }

    #[test]
    fn fibo_1() {
        assert_eq!(1, fibo(1));
    }

    #[test]
    fn fibo_2() {
        assert_eq!(1, fibo(2));
    }

    #[test]
    fn fibo_big() {
        assert_eq!(610, fibo(15));
    }
}

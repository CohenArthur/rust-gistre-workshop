pub fn fibo(v: u32) -> u32 {
    if v == 0 {
        0
    }
    else {
        let mut f0 = 0;
        let mut f1 = 1;
        let mut tmp;
        for _i in 1..v {
            tmp = f1;
            f1 += f0;
            f0 = tmp
        }
        f1
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

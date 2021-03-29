pub fn swap(a: &mut i32, b: &mut i32) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_basic() {
        let mut a = 15;
        let mut b = 16;

        swap(&mut a, &mut b);

        assert_eq!(a, 16);
        assert_eq!(b, 15);
    }
}

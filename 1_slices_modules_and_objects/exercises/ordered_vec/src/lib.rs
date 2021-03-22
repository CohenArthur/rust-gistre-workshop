pub struct OrderedVec {
    vec: Vec<i32>,
}

impl OrderedVec {
    /// Create a new, empty ordered vector
    pub fn new() -> OrderedVec {
        todo!()
    }

    /// Add an element to the vector, in order
    pub fn push(&mut self, value: i32) {
        todo!()
    }

    /// Remove the first element from the vector and return it
    pub fn pop(&mut self) -> i32 {
        todo!()
    }

    pub fn is_sorted(&self) -> bool {
        self.vec
            .as_slice()
            .windows(2)
            .all(|tuple| tuple[0] <= tuple[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = OrderedVec::new();
    }

    #[test]
    fn one_elt() {
        let mut ov = OrderedVec::new();
        ov.push(1);

        assert!(ov.is_sorted())
    }

    #[test]
    fn ordered_push() {
        let mut ov = OrderedVec::new();
        ov.push(1);
        ov.push(10);
        ov.push(100);
        ov.push(1000);

        assert!(ov.is_sorted())
    }

    #[test]
    fn reverse_order_push() {
        let mut ov = OrderedVec::new();
        ov.push(1000);
        ov.push(100);
        ov.push(10);
        ov.push(1);

        assert!(ov.is_sorted())
    }

    #[test]
    fn mix_match_push() {
        let mut ov = OrderedVec::new();
        ov.push(1);
        ov.push(100);
        ov.push(1000);
        ov.push(10);

        assert!(ov.is_sorted())
    }

    #[test]
    fn ordered_pop() {
        let mut ov = OrderedVec::new();
        ov.push(1);
        ov.push(100);
        ov.push(1000);
        ov.push(10);

        assert_eq!(ov.pop(), 1);
        assert_eq!(ov.pop(), 10);

        assert!(ov.is_sorted())
    }
}

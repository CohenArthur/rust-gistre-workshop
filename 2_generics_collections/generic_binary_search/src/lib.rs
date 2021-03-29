/// Finds the index of `value` in the sorted slice `v`
/// Note: `value` will always implement the `Copy` Trait
/// Returns the index if found, None otherwise
pub fn generic_binary_search<T: PartialOrd + Copy>(v: &[T], value: T) -> Option<usize> {
    let mut high: i32 = v.len() as i32 - 1;
    let mut low = 0;
    
    while high >= low {
        let mid = ((high - low)) / 2 + low;
        let mid_index = mid as usize;
        let cur = v[mid_index];

        if cur == value {
            return Some(mid_index);
        }
        if cur < value {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_empty() {
        let v = vec![];

        let index = generic_binary_search(&v, 42);

        assert_eq!(index, None);
    }

    #[test]
    fn find_one() {
        let v = vec![1];

        let index = generic_binary_search(&v, 1);

        assert_eq!(index, Some(0));
    }

    #[test]
    fn find_middle() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = generic_binary_search(&v, 15);

        assert_eq!(index, Some(3));
    }

    #[test]
    fn find_right() {
        let v = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        let index = generic_binary_search(&v, 'f');

        assert_eq!(index, Some(5));
    }

    #[test]
    fn find_left() {
        let v = vec![1, 7, 10, 15, 23, 40, 41, 50];

        let index = generic_binary_search(&v, 10);

        assert_eq!(index, Some(2));
    }

    #[test]
    fn find_edge_right() {
        let v = vec![-41, -23, -20, -15, -5, -4, 41];

        let index = generic_binary_search(&v, 41);

        assert_eq!(index, Some(6));
    }

    #[test]
    fn find_edge_left() {
        let v = vec!['a', 'c', 'e', 'g', 'h', 'l', 'q'];

        let index = generic_binary_search(&v, 'a');

        assert_eq!(index, Some(0));
    }

    #[test]
    fn find_not_there() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = generic_binary_search(&v, 14);

        assert_eq!(index, None);
    }

    #[test]
    fn find_edge_not_there_left() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = generic_binary_search(&v, 0);

        assert_eq!(index, None);
    }

    #[test]
    fn find_edge_not_there_right() {
        let v = vec![1, 7, 10, 15, 23, 40, 41];

        let index = generic_binary_search(&v, 20000);

        assert_eq!(index, None);
    }
}

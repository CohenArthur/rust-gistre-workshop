/// Create a sorted Vec from a `given` slice
pub fn merge_sort(given: &[i32]) -> Vec<i32> {
    if given.len() <= 1 {
        return Vec::from(given);
    }

    let splitted = given.split_at(given.len() / 2);
    
    merge(&merge_sort(splitted.0), &merge_sort(splitted.1))
}

/// Merge two sorted arrays into a new sorted Vec
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        res.push( if left[i] < right[j] { i += 1; left[i - 1] } else { j += 1; right[j - 1] } );
    }

    if i < left.len() {
        &res.copy_from_slice(&left[i..]);
    } else if j < right.len() {
        &res.copy_from_slice(&right[j..]);
    }

    res
}

pub fn is_sorted(v: &[i32]) -> bool {
    // Iterate over the slice 2 by 2, so [1, 2, 3] becomes (1, 2), (2, 3)...
    // Ensure that ALL elements (pairs) match the predicate
    v.windows(2).all(|tuple| tuple[0] <= tuple[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty() {
        let mut v = vec![];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }

    #[test]
    fn sort_one() {
        let mut v = vec![1];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }

    #[test]
    fn sort_sorted_two() {
        let mut v = vec![1, 2];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }

    #[test]
    fn sort_unsorted_two() {
        let mut v = vec![2, 1];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }

    #[test]
    fn sort_unsort_three() {
        let mut v = vec![1, 3, 2];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }

    #[test]
    fn sort_many() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }

    #[test]
    fn sort_many_unsorted() {
        let mut v = vec![15, 2, 1, 45, 6, 7, 8, 3, 3];

        let actual = merge_sort(&mut v);

        assert!(is_sorted(&actual));
    }
}

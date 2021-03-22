pub fn bubble_sort(v: &mut Vec<i32>) {
    let len = v.len(); //pour ne pas recalculer la len a chaque fois dans la boucle
    for i in 0..len {
        for j in 0..len - i - 1 {
            if v[j] > v[j + 1] {
                let tmp = v[j];     //utiliser v.swap(j, j + 1) ?
                v[j] = v[j + 1];
                v[j + 1] = tmp;
            }
        }
    }
}

pub fn is_sorted(v: &Vec<i32>) -> bool {
    // // The hard to read version :D
    // fn is_sorted_inner(last: i32, idx: usize, v: &[i32]) -> bool {
    //     match v {
    //         [] => true,
    //         [last_elt] => last <= *last_elt,
    //         [tuple_first, tuple_sec] => last <= *tuple_first && tuple_first <= tuple_sec,
    //         [tuple_first, tuple_sec, ..] => tuple_first <= tuple_sec && is_sorted_inner(*tuple_sec, idx + 1, &v[idx..]),
    //     }
    // }

    // match v.len() {
    //     0 => true,
    //     _ => is_sorted_inner(v[0], 0, &v[1..])
    // }

    v.as_slice() // Take a slice (&[i32]) on the vector
        .windows(2) // Iterate over the slice 2 by 2, so [1, 2, 3] becomes (1, 2), (2, 3)...
        .all(|tuple| tuple[0] <= tuple[1]) // Ensure that ALL elements (pairs) match the predicate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty() {
        let mut v = vec![];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sort_one() {
        let mut v = vec![1];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sort_sorted_two() {
        let mut v = vec![1, 2];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sort_unsorted_two() {
        let mut v = vec![2, 1];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sort_three() {
        let mut v = vec![1, 2, 3];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sort_many() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sort_many_unsorted() {
        let mut v = vec![15, 2, 1, 45, 6, 7, 8, 3, 3];

        bubble_sort(&mut v);

        assert!(is_sorted(&v));
    }
}

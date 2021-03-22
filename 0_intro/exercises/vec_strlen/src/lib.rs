pub fn vec_strlen(vec: Vec<&str>) -> Vec<usize> {
    vec.iter().map(|i|  i.len()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_empty() {
        let vec = vec![];
        let empty: Vec<usize> = vec![];

        assert_eq!(empty, vec_strlen(vec));
    }

    #[test]
    fn strlen_one() {
        let vec = vec!["hey"];

        assert_eq!(vec![3], vec_strlen(vec));
    }

    #[test]
    fn strlen_one_empty() {
        let vec = vec![""];

        assert_eq!(vec![0], vec_strlen(vec));
    }

    #[test]
    fn strlen_many() {
        let vec = vec!["Hello", ",", "Rust", "!"];

        assert_eq!(vec![5, 1, 4, 1], vec_strlen(vec));
    }

    #[test]
    fn strlen_many_long() {
        let vec = vec!["GISTRE", "ELIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIITE"];

        assert_eq!(vec![6, 35], vec_strlen(vec));
    }
}

/// Create an acronym from `name`
/// Result may be written in uppercase as a bonus, not required
/// # Example
///
/// ```
/// use acronym::make_acronym;
///
/// let acronym = make_acronym("Portable Network Graphics");
/// assert_eq!(acronym, "PNG");
/// ```
// Side note: this doc example is also tested when running `cargo test`
pub fn make_acronym(name: &str) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acronym_png() {
        let acronym = make_acronym("Portable Network Graphics");

        assert_eq!(acronym.to_uppercase(), "PNG");
    }

    #[test]
    fn acronym_gif() {
        let acronym = make_acronym("Graphics Interchange Format");

        assert_eq!(acronym.to_uppercase(), "GIF");
    }

    #[test]
    fn acronym_asap() {
        let acronym = make_acronym("As Soon As Possible");

        assert_eq!(acronym.to_uppercase(), "ASAP");
    }

    #[test]
    fn acronym_bbc() {
        let acronym = make_acronym("British Broadcasting Corporation");

        assert_eq!(acronym.to_uppercase(), "BBC");
    }
}

pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    vec!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match_should_return_empty_vec() {
        let haystack = "
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.
          ";

        let empty: Vec<&str> = Vec::new();
        assert_eq!(search("azertyuio", haystack), empty);
    }

    #[test]
    fn several_match_should_return_several_vec_elements() {
        let haystack = "
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.
          ";

        assert_eq!(search("nobody", haystack), vec!("        I'm nobody! Who are you?", "        Are you nobody, too?"));
    }

}

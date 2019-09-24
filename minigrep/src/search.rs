#[derive(Debug, PartialEq)]
pub struct MatchingLine<'a> {
    pub content: &'a str,
    pub number: usize,
}

pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<MatchingLine<'a>> {
    let lines = haystack.lines();

    lines.enumerate()
        .filter(|line| line.1.contains(needle))
        .map(|line| MatchingLine { number: line.0, content: line.1 })
        .collect()
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

        let empty: Vec<MatchingLine> = Vec::new();
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

        assert_eq!(
            search("nobody", haystack),
            vec!(
                MatchingLine { number: 1, content: "        I'm nobody! Who are you?" },
                MatchingLine { number: 2, content: "        Are you nobody, too?" }
            )
        );
    }
}

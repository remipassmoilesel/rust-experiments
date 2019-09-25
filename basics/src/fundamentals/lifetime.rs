#[derive(Debug, PartialEq)]
pub struct StructWithLifetime<'a> {
    pub content: &'a str,
    pub number: usize,
}

// 'a indicate which argument StructWithLifetime depends on
pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<StructWithLifetime<'a>> {
    let lines = haystack.lines();

    lines
        .enumerate()
        .filter(|line| line.1.contains(needle))
        .map(|line| StructWithLifetime {
            number: line.0,
            content: line.1,
        })
        .collect()
}

pub fn main() {
    let haystack = "
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.
          ";

    println!("{:?}", search("nobody", haystack));
}

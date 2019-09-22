pub enum CliCommand<'a> {
    AddMemo {
        memo: &'a str,
        description: &'a str,
    },
    SearchMemo {
        query: &'a str,
    },
}

pub struct ArgumentParser {}

impl<'a> ArgumentParser {
    pub fn parse(args: Vec<String>) -> Option<CliCommand<'a>> {
        Some(CliCommand::SearchMemo {
            query: "fake query",
        })
    }
}

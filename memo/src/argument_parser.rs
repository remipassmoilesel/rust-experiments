pub enum CliCommand<'a> {
    AddMemo { memo: &'a str, description: &'a str },
    SearchMemo { query: &'a str },
    InvalidCommand { message: &'a str },
}

pub struct ArgumentParser {}

impl<'a> ArgumentParser {
    pub fn parse(args: Vec<String>) -> CliCommand<'a> {
        let clean_args: Vec<&str> = args.iter().map(|arg| arg.trim()).collect();

        match clean_args.get(1) {
            Some(&"add") => CliCommand::AddMemo { memo: "add", description: "description" },
            Some(&"search") => CliCommand::SearchMemo { query: "fake query" },
            _ => CliCommand::InvalidCommand { message: "Invalid command" }
        }
    }
}

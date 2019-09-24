#[derive(Debug, PartialEq)]
pub enum CliCommand {
    AddMemo { memo: String, description: String },
    SearchMemo { query: String },
    InvalidCommand { message: String },
}

pub struct ArgumentParser {}

impl ArgumentParser {
    pub fn parse(args: Vec<String>) -> CliCommand {
        let clean_args: Vec<&str> = args.iter().map(|arg| arg.trim()).collect();

        match clean_args.get(1) {
            Some(&"add") => match clean_args {
                ref x if x.len() > 3 => CliCommand::AddMemo {
                    memo: String::from(*x.get(2).unwrap()),
                    description: String::from(*x.get(3).unwrap()),
                },
                _ => CliCommand::InvalidCommand {
                    message: String::from("You must specify a memo and a description"),
                },
            },
            Some(&"search") => match clean_args {
                ref x if x.len() > 2 => CliCommand::SearchMemo {
                    query: String::from(*x.get(2).unwrap()),
                },
                _ => CliCommand::InvalidCommand {
                    message: String::from("You must specify a query"),
                },
            },
            _ => CliCommand::InvalidCommand {
                message: String::from("Invalid command, try memo --help"),
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arguments_should_return_error() {
        assert_eq!(ArgumentParser::parse(vec!()), CliCommand::InvalidCommand { message: String::from("Invalid command, try memo --help") });
    }
}

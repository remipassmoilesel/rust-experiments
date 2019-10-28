use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(mut args: Iter<String>) -> Result<Config, &'static str> {
        args.next(); // skip bin path

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Query parameter is mandatory"),
        };
        let file_path = match args.next() {
            Some(q) => q,
            None => return Err("File path parameter is mandatory"),
        };

        Ok(Config {
            query: query.clone(),
            file_path: file_path.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_query_parameter_argument_should_return_error() {
        assert_eq!(
            Config::new(vec!().iter()),
            Err("Query parameter is mandatory")
        );
    }

    #[test]
    fn missing_path_parameter_argument_should_return_error() {
        let test_args = vec![String::from("/path/to/bin"), String::from("needle")];

        assert_eq!(
            Config::new(test_args.iter()),
            Err("File path parameter is mandatory")
        );
    }

    #[test]
    fn sufficient_arguments_should_return_config() {
        let test_args = vec![
            String::from("/path/to/bin"),
            String::from("needle"),
            String::from("haystack.txt"),
        ];

        assert_eq!(
            Config::new(test_args.iter()),
            Ok(Config {
                query: String::from("needle"),
                file_path: String::from("haystack.txt"),
            })
        );
    }
}

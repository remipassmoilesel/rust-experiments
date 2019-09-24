#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        match args {
            ref x if x.len() > 2 => Ok(Config {
                query: x[1].clone(),
                filename: x[2].clone(),
            }),
            _ => Err("Invalid number of arguments"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_few_arguments_should_return_error() {
        assert_eq!(Config::new(vec!()), Err("Invalid number of arguments"));
    }

    #[test]
    fn sufficients_arguments_should_return_config() {
        assert_eq!(
            Config::new(vec!(
                String::from("/path/to/bin"),
                String::from("needle"),
                String::from("haystack.txt")
            )),
            Ok(Config {
                query: String::from("needle"),
                filename: String::from("haystack.txt")
            })
        );
    }
}

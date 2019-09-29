#[derive(Debug)]
pub struct Configuration;

impl Configuration {
    pub fn new(path: String) ->  Result<Configuration, ConfigurationLoadError> {
        Ok(Configuration)
    }
}

#[derive(Debug)]
pub struct ConfigurationLoadError;

use serde_yaml;
use std::collections::HashMap;
use std::env;

/// Holds configuration parameters for the application
///
/// # Attributes
/// * map (HashMap<String, serde_yaml::Value>): the map of parameters for the app
pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    /// Creates a new Config map with loaded data from a yml file where the path is from the last argument
    /// passed into the program.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (Config): a loaded Config struct
    #[cfg(not(test))]
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
        return Config { map };
    }

    #[cfg(test)]
    pub fn new() -> Config {
        let mut map = HashMap::new();
        map.insert(
            String::from("DB_URL"),
            serde_yaml::from_str("postgres://username:password@localhost:5433/to_do").unwrap(),
        );
        map.insert(
            String::from("SECRET_KEY"),
            serde_yaml::from_str("secret").unwrap(),
        );
        map.insert(
            String::from("EXPIRE_MINUTES"),
            serde_yaml::from_str("120").unwrap(),
        );
        map.insert(
            String::from("REDIS_URL"),
            serde_yaml::from_str("redis://127.0.0.1/").unwrap(),
        );
        return Config { map };
    }
}

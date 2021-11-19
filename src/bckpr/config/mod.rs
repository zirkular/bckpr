extern crate yaml_rust;

use std::fs;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

pub enum CLI {
    Borgmatic,
    Restic,
    Unknown
}

pub struct Retention {
    pub keep_daily: i64,
    pub keep_weekly: i64,
    pub keep_monthly: i64,
}

impl Default for Retention {
    fn default() -> Self {
        Retention {
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 6,
        }
    }
}

pub struct Spinoff {
    pub cli: CLI,
    pub retention: Retention,
}

pub struct Config {
    pub spinoffs: Vec<Spinoff>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            spinoffs: Vec::new()
        }
    }
}

mod yaml {
    use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

    pub fn cli(node: &Yaml) -> &Yaml {
        &node["cli"]
    }

    pub fn source_directories(node: &Yaml) -> &Yaml {
        &node["source_directories"]
    }

    pub fn repositories(node: &Yaml) -> &Yaml {
        &node["repositories"]
    }

    pub fn retention(node: &Yaml) -> &Yaml {
        &node["retention"]
    }

    pub fn retention_keep_daily(node: &Yaml) -> &Yaml {
        &node["retention"]["keep_daily"]
    }

    pub fn retention_keep_weekly(node: &Yaml) -> &Yaml {
        &node["retention"]["keep_weekly"]
    }

    pub fn retention_keep_monthly(node: &Yaml) -> &Yaml {
        &node["retention"]["keep_monthly"]
    }
}

pub fn read_yaml(filename: &str) -> Vec<Yaml> {
    println!("Reading config file {:?}.", filename);
    let content = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    YamlLoader::load_from_str(&content).unwrap()
}

fn validate(node: &Yaml) -> Result<bool, &str> {
    println!("Validating config.");

    if yaml::cli(node).is_null() || yaml::cli(node).as_str() == None {
        return Err("Node 'cli' not found or invalid value type (required: String).");
    }

    if yaml::source_directories(node).is_null() ||
        !yaml::source_directories(node).is_array() {
        return Err("Node 'source_directories' not found or invalid value type +
            (required: Array).");
    }

    if yaml::repositories(node).is_null() || !yaml::repositories(node).is_array() {
        return Err("Node 'repositories' not found or invalid value type (required: Array).");
    }


    if !yaml::repositories(node).is_null() {
        if yaml::retention_keep_daily(node).is_null() &&
            yaml::retention_keep_weekly(node).is_null() &&
            yaml::retention_keep_monthly(node).is_null() {
            return Err("If retention is defined, at least one of the following values need to be existent:");
        } else {
            if !yaml::retention_keep_daily(node).is_null() &&
                yaml::retention_keep_daily(node).as_i64() == None {
                return Err("Node 'keep_daily' not found in node 'retention' or invalid value type (required: Integer).");
            }

            if !yaml::retention_keep_weekly(node).is_null() &&
                yaml::retention_keep_weekly(node).as_i64() == None {
                return Err("Node 'keep_weekly' not found in node 'retention' or invalid value type (required: Integer).");
            }

            if !yaml::retention_keep_monthly(node).is_null() &&
                yaml::retention_keep_monthly(node).as_i64() == None {
                return Err("Node 'keep_monthly' not found in node 'retention' or invalid value type (required: Integer).");
            }
        }

    }
    Ok(true)
}

pub fn parse_config(yaml: &Vec<Yaml>) -> Result<Config, &str> {
    let mut config: Config = Default::default();  
    let mut index = 0;  
    let entries = &yaml[0]["spinoffs"];

    while entries[index] != Yaml::BadValue {
        let entry = &entries[index];
        match *entry {
            Yaml::Hash(ref hash) => {
                for (key, value) in hash {
                    println!("Parsing spinoff {:?}.", key.as_str().unwrap());
                    
                    match validate(value) {
                        Ok(is_valid) => {},
                        Err(error) => {
                            return Err(error);
                        },
                    }

                    let retention: Retention = Default::default();                    
                    let spinoff = Spinoff {
                        cli: match yaml::cli(value).as_str().unwrap() {
                            "borgmatic" => CLI::Borgmatic,
                            "restic" => CLI::Restic,
                            _ => CLI::Unknown,
                        },
                        retention: Retention {
                            keep_daily: match yaml::retention_keep_daily(value).as_i64() {
                                None => retention.keep_daily,
                                Some(value) => value,
                            },
                            keep_weekly: match yaml::retention_keep_weekly(value).as_i64() {
                                None => retention.keep_weekly,
                                Some(value) => value,
                            },
                            keep_monthly: match yaml::retention_keep_monthly(value).as_i64() {
                                None => retention.keep_monthly,
                                Some(value) => value,
                            },
                        }
                    };
                    config.spinoffs.push(spinoff);
                }
            }
            _ => {
                println!("{:?}", entry);
            }
        }
        index = index + 1
    } 
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_full() {
        let yaml = read_yaml("test/config_valid_full.yaml");
        let config = parse_config(&yaml);
        assert!(!config.is_err());
    }

    #[test]
    fn invalid_missing_cli() {
        let yaml = read_yaml("test/config_invalid_missing_cli.yaml");
        let config = parse_config(&yaml);
        assert!(config.is_err());
    }

    #[test]
    fn invalid_missing_source_directories() {
        let yaml = read_yaml("test/config_invalid_missing_source_directories.yaml");
        let config = parse_config(&yaml);
        assert!(config.is_err());
    }

    #[test]
    fn invalid_missing_repositories() {
        let yaml = read_yaml("test/config_invalid_missing_repositories.yaml");
        let config = parse_config(&yaml);
        assert!(config.is_err());
    }

    #[test]
    fn invalid_retention_empty() {
        let yaml = read_yaml("test/config_invalid_retention_empty.yaml");
        let config = parse_config(&yaml);
        assert!(config.is_err());
    }

    #[test]
    fn invalid_cli_wrong_value_type() {
        let yaml = read_yaml("test/config_invalid_cli_wrong_value_type.yaml");
        let config = parse_config(&yaml);
        assert!(config.is_err());
    }
}
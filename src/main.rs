use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    environment: HashMap<String, String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options: HashMap<String, String> = HashMap::new();
    let mut input_env_variables: HashMap<String, String> = HashMap::new();
    let mut option_key: Option<String> = None;
    let mut env_key: Option<String> = None;
    let mut config_file: String = String::from("./config.json");

    for i in 1..args.len() {
        let arg = &args[i];
        let mut chars = arg.chars();
        let value = chars.clone().collect::<String>();

        if value.contains("--") && env_key.is_none() {
            env_key = Some(value.replace("--", ""));
            option_key = None;
        } else if value.contains("-") && option_key.is_none() {
            option_key = Some(value.replace("-", ""));
            env_key = None;
        } else if option_key.is_some() {
            options.insert(option_key.clone().unwrap(), value);
            option_key = None;
            env_key = None;
        } else if env_key.is_some() {
            input_env_variables.insert(env_key.clone().unwrap(), value);
            option_key = None;
            env_key = None;
        }
    }

    if options.contains_key("config") {
        config_file = options.get("config").unwrap().clone();
    }

    if config_file.contains(".yaml") {
        let config_contents = read_to_string(config_file).unwrap();
        let yaml_config: ConfigFile = serde_yaml::from_str(&config_contents).unwrap();

        for (key, value) in yaml_config.environment {
            input_env_variables.insert(key, value);
        }
    }

    println!("{:?}", options);
    println!("{:?}", input_env_variables);
}

use clap::Parser;
use config::Config;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Args {
    cmd: Option<String>,
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    env: Option<Vec<String>>,
}

fn parse_dot_env_file(file: String) -> HashMap<String, String> {
    dotenvy::from_filename(file).ok();
    let env_variables: HashMap<String, String> = dotenvy::vars().collect();

    env_variables
}

fn parse_env_file(file: String) -> HashMap<String, String> {
    let ext = Path::new(file.as_str())
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    match ext {
        "env" => parse_dot_env_file(file),
        "yaml" | "yml" | "json" | "json5" | "toml" | "ron" | "ini" => {
            let settings = Config::builder()
                .add_source(config::File::with_name(file.as_str()))
                .add_source(config::Environment::default())
                .build()
                .unwrap();

            settings.try_deserialize().unwrap()
        }
        _ => {
            println!("Unsupported file type");
            HashMap::new()
        }
    }
}

fn main() {
    let args = Args::parse();
    let env_file = args.file;
    let mut input_env_variables: HashMap<String, String> = HashMap::new();

    if env_file.is_some() {
        input_env_variables = parse_env_file(env_file.unwrap());
    }

    if args.env.is_some() {
        for env in args.env.unwrap() {
            let env_split: Vec<&str> = env.split('=').collect();
            input_env_variables.insert(
                env_split[0].to_string(),
                env_split[1..].join("=").to_string(),
            );
        }
    }

    match args.cmd {
        Some(cmd) => {
            Command::new(cmd)
                .envs(input_env_variables)
                .spawn()
                .map_err(|e| println!("Failed to execute process: {}", e))
                .expect("Failed to execute command");

            return;
        }
        None => {
            println!("No command provided");
            return;
        }
    }
}

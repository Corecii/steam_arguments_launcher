#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::fs::OpenOptions;
use std::env;
use std::io::prelude::*;
use std::process::Command;
use std::path::PathBuf;

#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
struct Config {
    exe_name: String,
    steam_path: String,
    steam_gameid: String,
    arguments: Vec<String>,
    debug: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            exe_name: String::new(),
            steam_path: String::new(),
            steam_gameid: String::new(),
            arguments: vec![],
            debug: false,
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
enum ConfigState {
    Found(Config, PathBuf),
    NotFound(PathBuf),
    Error
}

enum ConfigureUserResult {
    Configured,
    Error,
    NotConfigured
}

fn find_config() -> ConfigState {
    let mut exe_path = PathBuf::from(env::args().nth(0).unwrap());
    exe_path.pop();
    let mut config_path = exe_path.clone();
    config_path.push("app_steam_config");
    if !config_path.exists() {
        println!("Cannot find config file at `{}`", config_path.to_string_lossy());
        return ConfigState::NotFound(config_path);
    } else if !config_path.is_file() {
        println!("Found config, but it's not a file at `{}`", config_path.to_string_lossy());
        return ConfigState::Error;
    }
    return match read_config(&config_path) {
        Ok(config) => ConfigState::Found(config, config_path),
        Err(err) => {
            println!("{}", err);
            ConfigState::Error
        }
    }
}

fn read_config(config_path: &PathBuf) -> Result<Config, String> {
    return match OpenOptions::new().read(true).write(false).open(config_path) {
        Ok(mut read_file) => {
            let mut contents = String::new();
            match read_file.read_to_string(&mut contents) {
                Ok(_) => {
                    match serde_json::from_str(&contents) {
                        Ok(config) => Ok(config),
                        Err(err) => Err(format!("Malformed config file at `{}`\n{}", config_path.to_string_lossy(), err))
                    }
                },
                Err(err) => Err(format!("Could not read config file at `{}`\n{}", config_path.to_string_lossy(), err))
            }
        },
        Err(err) => Err(format!("Could not open config file for reading at `{}`\n{}", config_path.to_string_lossy(), err))
    }
}

fn make_config(config_path: PathBuf) -> ConfigureUserResult {
    println!("Launcher configuration:\n\t[1] Configure\n\t[2] Exit");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "2" => return ConfigureUserResult::NotConfigured,
            "1" => break,
            _ => println!("Please enter only 1 or 2.")
        }
    }
    println!("Configurator");
    let mut config = Config::new();
    println!("Enter application name:");
    let mut exe_name = String::new();
    std::io::stdin().read_line(&mut exe_name).expect("Failed to read line");
    config.exe_name = String::from(exe_name.trim());
    println!("Enter Steam executable path:");
    let mut steam_path = String::new();
    std::io::stdin().read_line(&mut steam_path).expect("Failed to read line");
    config.steam_path = String::from(steam_path.trim());
    println!("Enter application's Steam game id:");
    let mut steam_gameid = String::new();
    std::io::stdin().read_line(&mut steam_gameid).expect("Failed to read line");
    config.steam_gameid = String::from(steam_gameid.trim());


    match OpenOptions::new().read(false).write(true).create_new(true).open(&config_path) {
        Ok(mut write_file) => {
            let config_as_str = match serde_json::to_string(&config) {
                Ok(as_str) => as_str,
                Err(err) => {
                    println!("Could not serialize config struct.\n{}", err);
                    return ConfigureUserResult::Error;
                }
            };
            match write_file.write_all(config_as_str.as_bytes()) {
                Ok(_) => {
                    println!("Configuration written.\nThe application will now launch through Steam when ran.\nDelete the configuration file to reconfigure.");
                    return ConfigureUserResult::Configured;
                },
                Err(err) => {
                    println!("Could not write configuration.\n{}", err);
                    return ConfigureUserResult::Error;
                }
            };
        },
        Err(err) => {
            println!("Could not create config file at `{}`\n{}", config_path.to_string_lossy(), err);
            return ConfigureUserResult::Error;
        }
    }
}

fn write_arguments(config: &Config, config_path: &PathBuf, arguments: Vec<String>) ->ConfigureUserResult {
    match OpenOptions::new().read(false).write(true).create(false).truncate(true).open(config_path) {
        Ok(mut write_file) => {
            let mut config_new = config.clone();
            config_new.arguments = arguments;
            let config_as_str = match serde_json::to_string(&config_new) {
                Ok(as_str) => as_str,
                Err(err) => {
                    println!("Could not serialize config struct.\n{}", err);
                    return ConfigureUserResult::Error;
                }
            };
            match write_file.write_all(config_as_str.as_bytes()) {
                Ok(_) => {
                    if config.debug {
                        println!("Wrote arguments to configuration.");
                    }
                    return ConfigureUserResult::Configured;
                },
                Err(err) => {
                    println!("Could not write configuration.\n{}", err);
                    return ConfigureUserResult::Error;
                }
            };
        },
        Err(err) => {
            println!("Could not create config file at `{}`\n{}", config_path.to_string_lossy(), err);
            return ConfigureUserResult::Error;
        }
    }
}

fn run_game(config: &Config, config_path: &PathBuf) {
    write_arguments(config, config_path, vec![]);
    match Command::new(&config.exe_name)
        .args(&config.arguments)
        .spawn() {
            Ok(_) => if config.debug {
                println!("Started application successfully.");
            },
            Err(info) => {
                println!("Failed to run application!");
                println!("Reason: {}", info);
            }
        }
    if config.debug {
        println!("Press any key to exit.");
        std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    }
}

fn main() {
    let is_found = find_config();
    match is_found {
        ConfigState::Error => {
            println!("There was an error running the launcher.\nPress any key to continue.");
            std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
            return;
        },
        ConfigState::Found(config, config_path) => {
            if config.debug {
                println!("{}", env::args().fold(String::new(), |acc, arg| acc + "\n" + &arg));
            }
            match env::args().nth(1) {
                Some(arg) => {
                    if arg == String::from("-steam_game_launch") {
                        run_game(&config, &config_path);
                        return;
                    }
                },
                None => ()
            }
            write_arguments(&config, &config_path, env::args().collect::<Vec<_>>().split_off(1));
            match Command::new("explorer.exe")
                .arg(format!("steam://rungameid/{}", config.steam_gameid))
                .spawn() {
                    Ok(_) => if config.debug {
                        println!("Started Steam successfully.");
                    },
                    Err(info) => {
                        println!("Failed to run Steam!");
                        println!("Reason: {}", info);
                    }
                }
            if config.debug {
                println!("Press any key to exit.");
                std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
            }
        },
        ConfigState::NotFound(config_path) => {
            match make_config(config_path) {
                ConfigureUserResult::Configured | ConfigureUserResult::Error => {
                    println!("Press any key to exit.");
                    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
                },
                ConfigureUserResult::NotConfigured => ()
            }
        }
    }
}

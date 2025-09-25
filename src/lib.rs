use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    process::{Command, Stdio},
};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;

#[derive(Deserialize)]
struct Config {
    prefix: String,
    map: HashMap<String, Entry>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: ":cc".to_string(),
            map: HashMap::new(),
        }
    }
}

#[derive(Default, Deserialize)]
struct Entry {
    description: String,
    exec: String,
    envs: Option<Vec<(String, String)>>,
    print_output: Option<bool>,
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/custom-command.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_else(|why| {
            eprintln!("[custom-command] Failed to load config: {why}");
            Config::default()
        }),
        Err(e) => {
            eprintln!("[custom-command] Failed to read config file: {e}");
            Config::default()
        }
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Custom command".into(),
        icon: "help-about".into(), // Icon from the icon theme
    }
}

#[get_matches]
fn get_matches(input: RString, config: &mut Config) -> RVec<Match> {
    let input = if let Some(input) = input.strip_prefix(&config.prefix) {
        input.trim()
    } else {
        return RVec::new();
    };
    let mut matches = Vec::<Match>::new();

    for (key, value) in config.map.iter() {
        if key.contains(input) || value.description.contains(input) {
            matches.push(Match {
                title: key.clone().into(),
                description: ROption::RSome(value.description.clone().into()),
                icon: ROption::RNone,
                use_pango: false,
                id: ROption::RNone,
            });
        }
    }

    matches.into()
}

#[handler]
fn handler(selection: Match, config: &mut Config) -> HandleResult {
    let entry = config.map.get(selection.title.as_str()).unwrap();

    let mut split: Vec<&str> = entry.exec.split(" ").collect();
    if split.is_empty() {
        return HandleResult::Close;
    }

    let mut command = Command::new(split.first().unwrap());
    split.remove(0);
    command.args(split);

    if !entry.print_output.unwrap_or(false) {
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
    }

    if let Some(envs) = &entry.envs {
        for env in envs.iter() {
            command.env(env.0.clone(), env.1.clone());
        }
    }

    // A zombie process is exactly what we want
    #[allow(clippy::zombie_processes)]
    command.spawn().unwrap();

    HandleResult::Close
}

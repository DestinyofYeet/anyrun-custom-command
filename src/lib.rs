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
    commands: Vec<Entry>,
}

impl Config {
    fn new() -> Self {
        Self {
            prefix: ":cc".to_string(),
            commands: Vec::new(),
        }
    }
}

struct State {
    config: Config,
    ids: HashMap<u64, Entry>,
    selected_sub: Option<Vec<Entry>>,
}

#[derive(Deserialize, Clone, Debug)]
struct Entry {
    title: String,
    description: String,
    exec: Option<String>,
    envs: Option<Vec<(String, String)>>,
    print_output: Option<bool>,
    subcommands: Option<Vec<Entry>>,
}

#[init]
fn init(config_dir: RString) -> State {
    let config = match fs::read_to_string(format!("{}/custom-command.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_else(|why| {
            eprintln!("[custom-command] Failed to load config: {why}");
            Config::new()
        }),
        Err(e) => {
            eprintln!("[custom-command] Failed to read config file: {e}");
            Config::new()
        }
    };

    State {
        config,
        ids: HashMap::new(),
        selected_sub: None,
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
fn get_matches(input: RString, state: &mut State) -> RVec<Match> {
    let input = if let Some(input) = input.strip_prefix(&state.config.prefix) {
        input.trim()
    } else {
        return RVec::new();
    };
    let mut matches = Vec::<Match>::new();

    fn format_entry(entry: &Entry, id: u64) -> Match {
        Match {
            title: format!(
                "{} {}",
                entry.title,
                entry
                    .subcommands
                    .as_ref()
                    .map(|val| format!("| {} Subcommands", val.len()))
                    .unwrap_or_default()
            )
            .into(),
            description: ROption::RSome(entry.description.clone().into()),
            icon: ROption::RNone,
            use_pango: false,
            id: ROption::RSome(id),
        }
    }

    if let Some(subcommands) = state.selected_sub.take() {
        for subs in subcommands.iter() {
            let id: u64 = (state.ids.len() + 1) as u64;
            matches.push(format_entry(subs, id));
            state.ids.insert(id, subs.clone());
        }

        return matches.into();
    }

    for entry in state.config.commands.iter() {
        if entry.title.contains(input) {
            let id: u64 = (state.ids.len() + 1) as u64;
            matches.push(format_entry(entry, id));

            state.ids.insert(id, entry.clone());
        }
    }

    matches.into()
}

#[handler]
fn handler(selection: Match, state: &mut State) -> HandleResult {
    let entry = state.ids.get(&selection.id.unwrap()).unwrap();

    let exec = match &entry.exec {
        None => {
            state.selected_sub = entry.subcommands.clone();
            return HandleResult::Refresh(true);
        }
        Some(value) => value,
    };

    let mut split: Vec<&str> = exec.split(" ").collect();
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

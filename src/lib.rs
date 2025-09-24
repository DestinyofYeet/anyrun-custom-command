use serde::Deserialize;
use std::{collections::HashMap, fs};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;

#[derive(Default, Deserialize)]
struct State {
    prefix: String,
    map: HashMap<String, String>,
}

#[init]
fn init(config_dir: RString) -> State {
    match fs::read_to_string(format!("{}/custom-command.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_else(|why| {
            eprintln!("[custom-command] Failed to load config: {why}");
            State::default()
        }),
        Err(e) => {
            eprintln!("[custom-command] Failed to read config file: {e}");
            State::default()
        }
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Custom command runner".into(),
        icon: "help-about".into(), // Icon from the icon theme
    }
}

#[get_matches]
fn get_matches(input: RString, state: &mut State) -> RVec<Match> {
    let input = if let Some(input) = input.strip_prefix(&state.prefix) {
        input.trim()
    } else {
        return RVec::new();
    };
    let mut matches = Vec::<Match>::new();

    for key in state.map.keys() {
        if key.contains(input) {
            matches.push(Match {
                title: key.clone().into(),
                description: ROption::RNone,
                icon: ROption::RNone,
                use_pango: false,
                id: ROption::RNone,
            });
        }
    }

    matches.into()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    HandleResult::Close
}

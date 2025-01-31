use serde::{Deserialize, Serialize};

pub const DEFAULT_REGEX_LIST: &str = "cfg/regx.txt";
pub const DEFAULT_STEAMID_LIST: &str = "cfg/steamids.txt";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub user: String,
    pub join_alert: bool,
    pub chat_reminders: bool,
    pub kick: bool,

    pub refresh_period: f32,
    pub kick_period: f32,
    pub alert_period: f32,

    pub rcon_password: String,
    pub tf2_directory: String,

    pub record_steamids: bool,
    pub steamid_list: String,
    pub regex_list: String,

    pub steamid_lists: Vec<String>,
    pub regex_lists: Vec<String>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            user: String::from("U:1:XXXXXXX"),
            join_alert: false,
            chat_reminders: false,
            kick: true,

            refresh_period: 10.0,
            kick_period: 10.0,
            alert_period: 20.0,

            rcon_password: String::from("tf2bk"),
            tf2_directory: String::new(),

            record_steamids: true,
            steamid_list: String::from(DEFAULT_STEAMID_LIST),
            regex_list: String::from(DEFAULT_REGEX_LIST),

            steamid_lists: vec![DEFAULT_STEAMID_LIST.to_string()],
            regex_lists: vec![DEFAULT_REGEX_LIST.to_string()],
        }
    }

    pub fn import(file: &str) -> Result<Settings, Box<dyn std::error::Error>> {
        return match std::fs::read_to_string(file) {
            Ok(contents) => match serde_json::from_str::<Settings>(&contents) {
                Ok(set) => Ok(set),
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        };
    }

    pub fn export(&self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        return match serde_json::to_string(self) {
            Ok(contents) => match std::fs::write(file, &contents) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        };
    }
}

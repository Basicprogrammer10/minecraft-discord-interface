use simple_config_parser::Config as Cfg;

macro_rules! cfg_get {
    ($cfg:expr, $name:expr) => {
        $cfg.get_str($name)
            .expect(concat!("Error getting `", $name, "` from Config"))
    };

    ($cfg:expr, $name:expr, $parse_type:ty) => {
        $cfg.get::<$parse_type>($name)
            .expect(concat!("Error getting `", $name, "` from Config"))
    };
}

#[derive(Debug, Clone)]
pub struct Config {
    pub raw_data: Vec<[String; 2]>,
    pub bot: BotConfig,
    pub minecraft: MinecraftConfig,
}

#[derive(Debug, Clone)]
pub struct BotConfig {
    pub token: String,
    pub data_channel: u64,
    pub event_channel: u64,
    pub command_prefix: String,
    pub message_id_path: String,
    pub disabled_commands: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MinecraftConfig {
    pub dir: String,
    pub java_path: String,
    pub start_cmd: String,
    pub disabled_events: Vec<String>,
}

impl Config {
    pub fn new(cfg: Cfg) -> Self {
        Self {
            bot: BotConfig::new(&cfg),
            minecraft: MinecraftConfig::new(&cfg),
            raw_data: cfg.data,
        }
    }
}

impl BotConfig {
    pub fn new(cfg: &Cfg) -> Self {
        Self {
            token: cfg_get!(cfg, "bot_token"),
            data_channel: cfg_get!(cfg, "bot_data_channel", u64),
            event_channel: cfg_get!(cfg, "bot_event_channel", u64),
            command_prefix: cfg_get!(cfg, "bot_command_prefix"),
            message_id_path: cfg_get!(cfg, "data_message_id_file"),
            disabled_commands: parse_string_arr(cfg_get!(cfg, "bot_disabled_commands")),
        }
    }
}

impl MinecraftConfig {
    pub fn new(cfg: &Cfg) -> Self {
        Self {
            dir: cfg_get!(cfg, "mc_dir"),
            java_path: cfg_get!(cfg, "mc_java_path"),
            start_cmd: cfg_get!(cfg, "mc_start_cmd"),
            disabled_events: parse_string_arr(cfg_get!(cfg, "mc_disabled_events")),
        }
    }
}

#[inline]
fn parse_string_arr(str: String) -> Vec<String> {
    str.split(',').map(|x| x.trim().to_owned()).collect()
}

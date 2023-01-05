extern crate serde_derive;
use self::serde_derive::{Serialize, Deserialize};
extern crate toml;


#[derive(Serialize, Deserialize)]
pub struct SonicPiToolCfg {
    pub token: i32,
    pub sonic_pi_port: u16,
    pub daemon_port: u16,
    pub gui_port: u16
}
impl SonicPiToolCfg {
    pub fn new(token: i32, sonic_pi_port: u16, daemon_port: u16, gui_port: u16) -> Self {
        Self {
            token,
            sonic_pi_port,
            daemon_port,
            gui_port
        }
    }

    pub fn read_from_path<P>(path: P) -> Self where P: AsRef<std::path::Path> {
        let bytes = std::fs::read(path).unwrap();
        toml::from_slice(&bytes).unwrap()
    }

    pub fn get_default_cfg_folder() -> String {
        let home = std::env::var("HOME").unwrap();
        format!("{}/.sonic-pi/tool/", home)
    }
    pub fn get_default_cfg_file_path() -> String {
        format!("{}/config.toml", Self::get_default_cfg_folder())
    }
}

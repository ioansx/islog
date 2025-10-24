const DEFAULT_XDG_DATA_HOME_DIR: &str = ".local/share";

pub fn default_xdg_data_home(home_dir: &str) -> String {
    format!("{}/{DEFAULT_XDG_DATA_HOME_DIR}", home_dir)
}

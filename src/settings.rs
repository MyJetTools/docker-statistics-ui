pub struct SettingsModel;

impl SettingsModel {
    pub fn get_src_urls(&self) -> Vec<String> {
        let urls = read_env_variable("SRC_URLS");
        urls.split(";").map(|s| s.to_string()).collect()
    }
}

fn read_env_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(url) => return url,
        Err(_) => panic!("{} is not set", name),
    }
}

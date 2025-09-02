use std::fs;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub ping_test_ip: String,
    pub login_url: String,
    pub account: String,
    pub isp: String,
    pub password: String,
}

impl Parameters {
    pub fn new()->Self{
        let path = Path::new("settings.yaml");

        if path.exists() {
            let content = fs::read_to_string(path).expect("settings.yaml file read error");
            serde_yaml::from_str(&content).expect("YAML parse error")
        } else {
            let default = Parameters{
                ping_test_ip:"119.29.29.29".to_string(),
                login_url:"http://1.2.3.4".to_string(),
                account:"your_account".to_string(),
                isp: "yidong".to_string(),
                password: "your_password".to_string(),
            };

            let yaml = serde_yaml::to_string(&default).expect("YAML serialization error");
            let mut file = fs::File::create(path).expect("settings.yaml creation error");
            file.write_all(yaml.as_bytes()).expect("settings.yaml write error");
            panic!("Modify `settings.yaml` after the first run.");
        }
    }
}


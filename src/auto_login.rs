use crate::parameters::Parameters;
use fantoccini::wd::{Capabilities, TimeoutConfiguration};
use fantoccini::{ClientBuilder, Locator};
use serde_json::json;
use std::time::Duration;

pub struct AutoLogin {
    param: Parameters,
}

impl AutoLogin {
    pub fn new(parameters: Parameters) -> Self {
        AutoLogin { param: parameters }
    }

    pub async fn login(&mut self) -> Result<(), fantoccini::error::CmdError> {
        let mut cap = Capabilities::default();

        if self.param.moz_firefox_options_binary.is_empty() {
            // Headless config
            cap.insert(
                "moz:firefoxOptions".to_string(),
                json!({
                    "args": ["--headless"],
                    "prefs": {
                        // Disable captive portal
                        "network.captive-portal-service.enabled": false,
                        "network.captive-portal-service.maxInterval": 0,
                        "network.connectivity-service.enabled": false
                    }
                }),
            );
        }
        else {
            // Headless config
            cap.insert(
                "moz:firefoxOptions".to_string(),
                json!({
                    "args": ["--headless"],
                    "binary":&self.param.moz_firefox_options_binary,
                    "prefs": {
                        // Disable captive portal
                        "network.captive-portal-service.enabled": false,
                        "network.captive-portal-service.maxInterval": 0,
                        "network.connectivity-service.enabled": false,
                    }
                }),
            );
        }

        let client = ClientBuilder::native()
            .capabilities(cap)
            .connect(&self.param.geckodriver_url)
            .await
            .expect("failed to connect to WebDriver");

        let time_out = TimeoutConfiguration::new(
            Some(Duration::from_secs(15)),
            Some(Duration::from_secs(15)),
            Some(Duration::from_secs(15)),
        );
        client
            .update_timeouts(time_out)
            .await
            .expect("failed to set webdrive timeout");

        tokio::time::sleep(Duration::from_secs(1)).await;

        _ = client.goto(&self.param.login_url).await;

        println!("Page loaded, URL: {}", client.current_url().await?);

        let user_name = client.find(Locator::Id("userName")).await?;
        let password = client.find(Locator::Id("password")).await?;

        println!("Send username: {}", self.param.account);
        user_name.send_keys(&self.param.account).await?;
        println!("Send password");
        password.send_keys(&self.param.password).await?;

        let yidong = client
            .find(Locator::XPath(
                "//input[@type='radio' and @name='operator' and @value='yd']",
            ))
            .await?;
        let liantong = client
            .find(Locator::XPath(
                "//input[@type='radio' and @name='operator' and @value='lt']",
            ))
            .await?;
        let dianxin = client
            .find(Locator::XPath(
                "//input[@type='radio' and @name='operator' and @value='dx']",
            ))
            .await?;

        if self.param.isp == "dianxin".to_string() {
            dianxin.click().await?;
        } else if self.param.isp == "yidong".to_string() {
            yidong.click().await?;
        } else if self.param.isp == "liantong".to_string() {
            liantong.click().await?;
        }

        let login_button = client.find(Locator::Id("loginBtn")).await?;

        tokio::time::sleep(Duration::from_secs(1)).await;

        login_button.click().await?;

        tokio::time::sleep(Duration::from_secs(5)).await;

        println!("Login result url: {}", client.current_url().await?);

        println!("Login complete");

        Ok(())
    }
}

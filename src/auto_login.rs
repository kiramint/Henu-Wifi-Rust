use std::time::Duration;
use crate::parameters::Parameters;
use fantoccini::wd::{Capabilities, TimeoutConfiguration};
use fantoccini::{ClientBuilder, Locator};
use serde_json::json;


pub struct AutoLogin {
    param: Parameters,
}

impl AutoLogin {
    pub fn new(parameters: Parameters) -> Self {
        AutoLogin { param: parameters }
    }

    pub async fn login(&mut self) -> Result<(), fantoccini::error::CmdError> {
        let mut cap = Capabilities::default();

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

        let client = ClientBuilder::native()
            .capabilities(cap)
            .connect("http://localhost:4444")
            .await
            .expect("failed to connect to WebDriver");

        let time_out = TimeoutConfiguration::new(
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(10))
        );
        client.update_timeouts(time_out).await.expect("failed to set webdrive timeout");

        client.goto(&self.param.login_url).await;

        println!("Page loaded, URL: {}", client.current_url().await.unwrap());

        let user_name = client.find(Locator::Id("userName")).await.unwrap();
        let password = client.find(Locator::Id("password")).await.unwrap();

        user_name.send_keys(&self.param.account).await.unwrap();
        password.send_keys(&self.param.password).await.unwrap();

        let yidong=client.find(Locator::XPath("//input[@type='radio' and @name='operator' and @value='yd']")).await.unwrap();
        let liantong = client.find(Locator::XPath("//input[@type='radio' and @name='operator' and @value='lt']")).await.unwrap();
        let dianxin = client.find(Locator::XPath("//input[@type='radio' and @name='operator' and @value='dx']")).await.unwrap();

        if self.param.isp == "dianxin".to_string() {
            dianxin.click().await.unwrap();
        } else if self.param.isp == "yidong".to_string() {
            yidong.click().await.unwrap();
        } else if self.param.isp == "liantong".to_string() {
            liantong.click().await.unwrap();
        }

        let login_button = client.find(Locator::Id("loginBtn")).await.unwrap();

        login_button.click().await.unwrap();

        tokio::time::sleep(Duration::from_secs(5)).await;

        println!("Login result url: {}",client.current_url().await.unwrap());

        Ok(())
    }
}

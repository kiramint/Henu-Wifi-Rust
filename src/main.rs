use std::net::{IpAddr, Ipv4Addr};
use std::process::{Command, Stdio};
use chrono::Local;
use tokio;

mod auto_login;
mod parameters;

#[tokio::main]
async fn main() {
    let mut err = 0;

    println!("Program start at {}",Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    println!("Getting parameters");
    let parameters = parameters::Parameters::new();
    let mut auto_login = auto_login::AutoLogin::new(parameters);

    loop {
        let payload = [0; 8];

        match surge_ping::ping(IpAddr::from(Ipv4Addr::new(119, 29, 29, 29)), &payload).await {
            Ok((_, _)) => {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                err = 0;
            }
            Err(e) => {
                if err < 5 {
                    println!("Ping failed {} times,at {}: {}", err,Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), e);
                    err += 1;
                } else {
                    println!("Starting geckodrive");
                    let _gecko = Command::new("./geckodriver")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .expect("Could not start geckodriver");

                    println!("Start auto login script");
                    auto_login.login().await.unwrap();
                    err = 0;
                }
            }
        }
    }
}

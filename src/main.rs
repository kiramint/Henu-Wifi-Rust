use tokio;
use chrono::Local;
use std::process::{exit, Command, Stdio};

mod auto_login;
mod parameters;

#[tokio::main]
async fn main() {
    let mut err = 0;

    println!(
        "Program start at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );

    println!("Note: This tool needs ping, but don't run it in root, because of firefox");

    println!("Getting parameters");
    let parameters = parameters::Parameters::new();
    let param = parameters.clone();

    println!("Account: {} ISP: {}",param.account,param.isp);
    println!("Gecko Path: {} Gecko URL: {}",param.geckodriver_path,param.geckodriver_url);

    let mut auto_login = auto_login::AutoLogin::new(parameters);

    loop {
        let payload = [0; 8];

        match surge_ping::ping(param.ping_test_ip.parse().expect("Not a valid ip address"), &payload).await {
            Ok((_, _)) => {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                err = 0;
            }
            Err(e) => {
                if err < 5 {
                    println!(
                        "Ping failed {} times, at {}: {}",
                        err,
                        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        e
                    );
                    if e.to_string().contains("Operation not permitted") {
                        println!("Ping failed, operation not permitted, try 'sudo sysctl -w net.ipv4.ping_group_range=\"0 2147483647\"
'");
                        exit(-1);
                    }
                    err += 1;
                } else {
                    println!("Starting geckodrive");
                    if !param.geckodriver_path.is_empty() {
                        let _gecko = Command::new(&param.geckodriver_path)
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn()
                            .expect("Could not start geckodriver");
                    } else {
                        println!("Using remote geckodriver at: {}",param.geckodriver_url);
                    }

                    println!("Start auto login script");
                    match auto_login.login().await {
                        Ok(_) => {
                            err = 0;
                        }
                        Err(e) => {
                            println!("Errors while trying to login, err: {}", e);
                        }
                    };
                }
            }
        }
    }
}

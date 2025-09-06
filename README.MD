# Henu Wifi Auto Login — Rust version

* A Firefox based auto login tool, Fantoccini + geckodrive

## Usage

* Install Firefox in your docker container or Linux device

* Download `geckodrive` and place it in the same directory of this program

* Run this tool once, it will generate `settings.yaml`

* Fill `settings.yaml`

  ```yaml
  geckodriver_path: ./geckodriver # Empty for remote geckodriver
  geckodriver_url: http://localhost:4444 # URL to connect geckodriver
  moz_firefox_options_binary: '' # Firefox bin path, in windows, it won't find automatically
  ping_test_ip: 119.29.29.29 # Best to use a DNS server
  login_url: http://1.1.1.1 # Will be redirect to portal page, 172.29.35.36
  account: your_account
  isp: yidong # isp: yidong/dianxin/liantong/henulocal
  password: your_password
  ```

* Run the tool, it will detect network every 10 sec, when ping fails 5 times, it will try to login

> [!IMPORTANT]
>
> * This tool needs ping privilege, you can try `sudo sysctl -w net.ipv4.ping_group_range="0 2147483647"` or use docker with `alpine:latest`
>   * run `apk install firefox` before use.
>
> * The network testing server is 119.29.29.29 and can’t be change, to modify, recompile is needed.
> * Do not modify `login_url` if you don’t know what you are doing, it will need the redirect get request to get your router’s ip address, and it seems only `http://1.1.1.1` works, otherwise firefox will be redirected to`about:blank`

## Build

```
cargo build --release
```

> [!IMPORTANT]
>
> * OpenSSL needs to disable in `cargo.toml` on windows

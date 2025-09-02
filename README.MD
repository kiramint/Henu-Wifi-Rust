# Henu Wifi Auto Login — Rust version

* A firefox based auto login tool, fantoccini + geckodrive

## Usage

* Install firefox in your docker container or linux device

* Download geckodrive and place it in the same directory of this program

* Run this tool once, it will generate `settings.yaml`

* Fill `settings.yaml`

  ```yaml
  ping_test_ip: 119.29.29.29 # deprecated
  login_url: http://1.1.1.1 # will be redirect to portal page, 172.29.35.36
  account: your_account
  isp: yidong # yidong/dianxin/liantong
  password: your_password
  ```

* Run the tool, it will detect network every 10 sec, when ping fails 5 times, it will try to login

> [!IMPORTANT]
>
> * The network testing server is 119.29.29.29 and can’t be change, to modify, recompile is needed.
>
> * Do not modify `login_url` if you don’t know what you are doing, it will need the redirect get request to get your router’s ip address.

## Build

```
cargo build --release
```


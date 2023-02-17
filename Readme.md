# CFScanner-rust
A Cross Platform CFScanner App ,Written In Rust

Main Script By Morteza Bashsiz:
[CFScanner](https://github.com/MortezaBashsiz/CFScanner/).

## Requirements

You just need to have [curl](https://curl.se/download.html) installed.

## Features

- Available on both Windows and Linux Operating Systems(Mac will be added later) .

- User can choose diffrent configs (located in `"servers"` folder) in the app (Only Trojan and Vmess are Supported).

- Using rust concurency system (No need to install parallel) .

## How It Works

Program has `five` main folders explained below:

1. `"servers"`: Put your client-config files into this folder.(Regular v2ray `client.json` configs).

2. `"ip_subnets"`: including ips of each Ok subnets to get ips locally without using `nmap` (OK subnets are listed in the `main.rs` file and it's synced with main script of [CFScanner](https://github.com/MortezaBashsiz/CFScanner/)).

3. `"result"`: including ok ips, each file contains list of ok ips with their response time (Unfortunately it's not in order).

4. `"v2ray"`: copy v2ray app to this directory, depends on your OS(Now,linux version is in this directory).

5. `"configs"`: including v2ray configs depends on the ip.(just like CFScanner script).

Note: you need to change `v2ray` and `servers` directories depending on your OS and clinet-configs.
### Execution

First, you should choose your config from config files located in the `"servers"` direcotry.


Next, choose number of threads depending on your system specs.

Note that on linux systems, Cloudflare ips can be obtained with `nmap` command, so you can choose weather to use it or not in the program.

Finally, all of the ok ips will store in the `"result"` directory.

Live Free!
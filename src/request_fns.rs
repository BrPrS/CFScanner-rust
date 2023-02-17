use std::{thread::{self,JoinHandle}, process::Command, time::{Duration, Instant}};
use colour::{green_ln, red_ln};

use crate::ip_fns::{create_port_from_ip};
use crate::fs_and_config_fns::write_ok_ips_in_results;
use std::sync::mpsc;
pub fn check_connection_through_v2ray(curl_installed:bool ,ip: &String)->Option<JoinHandle<()>> {
    let port = create_port_from_ip(ip);
    if curl_installed {
        let (tx, rx) = mpsc::channel();
        let handler = thread::spawn(move || {
            //curl -x "socks5://127.0.0.1:3$port" -s -w "TIME: %{time_total}\n" https://scan.sudoer.net
            let output = Command::new("curl")
                .args([
                    "-x",
                    format!("socks5://127.0.0.1:3{}", &port).as_str(),
                    "-s",
                    "-w",
                    "\"TIME: %{time_total}\n\"",
                    "https://scan.sudoer.net",
                ])
                .output();
            tx.send(output).ok();
        });

        let two_sec = Duration::from_secs(10);
        let output = rx.recv_timeout(two_sec);

        match output {
            Ok(result) => match result {
                Ok(result) => {
                    let output = String::from_utf8(result.stdout).unwrap();
                    if output.contains("TIME") {
                        
                        let start = output.trim().find("TIME").unwrap();
                        let time = &output[start..];
                        //Debug
                        // println!("{time}");
                        let time = time.strip_prefix("TIME: ").unwrap().trim();
                        let mut time = time.replace("\"", "");
                        time = time.trim().to_string();
                        if cfg!(target_os="windows"){
                            time = time.replace("/", ".")
                        }
                        let time: f64 = time.parse().unwrap();
                        let time_in_sec = (time.clone() * 1000.).round() as u32;
                        //FIXME Fix this range
                        if time_in_sec > 100 && time_in_sec < 10_000 {
                            if time_in_sec != 0 {
                                green_ln!("{} ResponseTime: {} ms  OK", &ip, &time_in_sec);
                                write_ok_ips_in_results(&ip, &time_in_sec.to_string());
                            }
                        }
                    }
                }
                Err(_) => {
                    print!("{}", ip);
                    red_ln!(" Failed")
                }
            },
            Err(_) => {
                print!("{}", ip);
                red_ln!(" Failed")
            }
        };
        return Some(handler);
    } else {
        let proxy = reqwest::Proxy::https("http://127.0.0.1:2081").unwrap();

    let client = reqwest::blocking::Client::builder()
            .proxy(proxy)
            .build().unwrap();
    let req =client.get("https://scan.sudoer.net");
    let time = Instant::now();
    let res=req.send().unwrap();
    if res.status().is_success(){
        let time_in_sec =time.elapsed().as_secs_f64()*1000.;
        let time_in_sec=time_in_sec as u32;
        if time_in_sec > 50 && time_in_sec < 10_000 {
            if time_in_sec != 0 {
                green_ln!("{} ResponseTime: {} ms  OK", &ip, &time_in_sec);
                write_ok_ips_in_results(&ip, &time_in_sec.to_string());
            }
        }
    }
    return None;    
    }

}

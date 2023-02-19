use colour::red_ln;
use std::{process::Command};
pub fn find_clf_ip_list()->Vec<String>{
    //curl -s -XGET https://www.cloudflare.com/ips-v4
    let output;
if cfg!(target_os = "windows") {
        output=Command::new("curl")
              .args(["-s","-XGET","https://www.cloudflare.com/ips-v4"])
              .output()
              .expect("failed to curl to find subnets");
        // println!("{:#?}",output);
        
  } else {
  
        output=Command::new("curl")
              .args(["-s","-XGET","https://www.cloudflare.com/ips-v4"])
              .output()
              .expect("failed to curl to find subnets");
  
  };
//   println!("{:#?}",output);

  let binding = String::from_utf8(output.stdout).expect("invalid output of curl command");
  let mut subnets=Vec::<String>::new();
  for subnet in binding.lines(){
    subnets.push(String::from(subnet));
  }
  // println!("{:#?}",subnets);

    subnets  

}

pub fn ips_in_subnet(subnet:String)->Vec<String>{
    // nmap -sL -n "$subNet" | awk '/Nmap scan report/{print $NF}'
    // ,"|","awk","/Nmap scan report/{print $NF}"

    let output=Command::new("nmap")
    .args(["-sL","-n",&subnet])
    .output()
    .expect("failed to curl to find subnets");
    let binding = String::from_utf8(output.stdout).expect("invalid output of curl command");
    let mut ips=Vec::<String>::new();
    for output in binding.lines(){
        if output.contains("Nmap scan report for"){
            let ip =String::from(output.strip_prefix("Nmap scan report for").expect("failed at removing extara dec at nmap"));
            //Debug
            // println!("{ip}");
            ips.push(ip);
            
        }

    }
    ips
}

use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
pub fn check_ip(ip: &String) -> (bool,JoinHandle<()>) {
    //curl -s -w "%{http_code}\n" --tlsv1.2 -servername fronting.sudoer.net -H "Host: fronting.sudoer.net" --resolve fronting.sudoer.net:443:203.29.52.233 https://fronting.sudoer.net | grep '200'
    //200
    let (tx , rx) = mpsc::channel();
    let _ip = String::from(ip.clone());
    //Debug
    // println!(
    //     "{}",
    //     format!("fronting.sudoer.net:443:{}", ip.trim()).as_str()
    // );
    let handle = thread::spawn(move || {
        
        let output = Command::new("curl")
            .args([
                "-s",
                "-w",
                "\"%{http_code}\n\"",
                "--tlsv1.2",
                "-servername",
                "fronting.sudoer.net",
                "Host: fronting.sudoer.net",
                "--resolve",
                format!("fronting.sudoer.net:443:{}", _ip.trim()).as_str(),
                "https://fronting.sudoer.net",
            ])
            .output()
            .expect("Failed to get output of Curl command");
        tx.send(output);
    });
    let two_sec = Duration::from_secs(2);

    let output = rx.recv_timeout(two_sec);
    // thread::sleep(two_sec);
    let result_status = match output {
        Ok(result) => {
            let result = String::from_utf8(result.stdout).unwrap();
            //Debug
            // println!("{:?}", result);

            if result.contains("200") {
                true
            } else {
                // print!("{}   ",&ip);
                // red_ln!("Failed");
                false
            }
        }
        Err(e) => {
            // print!("{}   ",&ip);
            // red_ln!("Failed");
            false
        },
    };

    // println!("{:#?}", result_status);
    return (result_status,handle)
}

pub fn create_port_from_ip(ip: &String) -> u16 {
  let temp = ip.split(".");
  let mut port: u16 = 0;
  for i in temp {
      port += i.trim().parse::<u16>().unwrap_or_default();
  }
  port
}
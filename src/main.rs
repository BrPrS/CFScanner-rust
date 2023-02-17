use colour::{blue, blue_ln, cyan_ln, green, green_ln, red, red_ln, yellow, yellow_ln};
use serde_json::Value;
mod fs_and_config_fns;
mod request_fns;
use crate::fs_and_config_fns::{read_temp_config, run_v2ray, write_config_with_ip, find_configs};
mod ip_fns;
use crate::ip_fns::{check_ip};
use crate::request_fns::check_connection_through_v2ray;
use std::process::{Command, exit};
use std::{thread, env};
use std::thread::JoinHandle;
use std::fs::{self,OpenOptions};

static CURL_INSTALLED: Lazy<bool> = Lazy::new(|| true);

static CLOUD_FLARE_OK_LIST:Lazy<Vec<u8>>=Lazy::new(||vec![31, 45, 66, 80, 89, 103, 104, 108, 141, 147, 154, 159, 168, 170, 185, 188, 191, 192, 193,
194, 195, 199, 203, 205, 212]);
static mut IPS:Lazy<Vec<String>> = Lazy::new(||get_ips());
static SUBNETS: Lazy<Vec<String>> =  Lazy::new(||ip_fns::find_clf_ip_list());
    

use std::sync::{Arc, Mutex};

fn each_thread_job(ip: &String, config_file: Value,is_curl_installed:bool)->JoinHandle<()> {
    let ip = Arc::new(Mutex::new(ip.clone()));
    let config_file = Arc::new(Mutex::new(config_file));
    let handle = thread::spawn(move || {
        let ip = ip.lock().unwrap();
        let config_file = config_file.lock().unwrap();
        if is_curl_installed{
        let (check_ip_reuslt, check_ip_handler) = check_ip(&ip);
        if check_ip_reuslt {
            write_config_with_ip(config_file.clone(), &ip);
            let mut handler = run_v2ray(format!("config.json.{}", ip.trim()).as_str());
            let check_connection_handler = check_connection_through_v2ray(is_curl_installed,&ip);
            if !cfg!(target_os="windows"){
                handler.kill().unwrap();
            }
            check_ip_handler.join().unwrap();
            if check_connection_handler.is_some(){
                check_connection_handler.unwrap().join().unwrap();
            }
    }
        }
        
    else{
        write_config_with_ip(config_file.clone(), &ip);
        let mut handler = run_v2ray(format!("config.json.{}", ip.trim()).as_str());
        let check_connection_handler = check_connection_through_v2ray(is_curl_installed,&ip);
        handler.kill().unwrap();
        if check_connection_handler.is_some(){
            check_connection_handler.unwrap().join().unwrap();
        }
    }
    
    });
    handle
}
fn kill_v2rays() {
    //for windows
    //taskkill /F /IM <processname.exe> /T
    if cfg!(target_os = "windows"){
    
        Command::new("taskkill")
        .args(["/F","/IM","v2ray.exe","/T"])
        .output()
        .expect("failed to stop v2rays");
    }
    else{
    Command::new("killall")
        .arg("v2ray")
        .output()
        .expect("failed to stop v2rays");
    }
}
fn kill_curls(){
    //taskkill /F /IM <processname.exe> /T
    if cfg!(target_os = "windows"){
        Command::new("taskkill")
        .args(["/F","/IM","curl","/T"])
        .output()
        .expect("failed to stop v2rays");
    }else{
    Command::new("killall")
    .arg("curl")
    .output()
    .expect("failed to stop v2rays");
    }
}


fn write_ip_subnets(subnet:&String,ips: &  Vec<String>){
    let path = env::current_dir().unwrap();
    let binding=subnet.find("/").unwrap();
    let subnet_1=&subnet[..binding];
    let subnet_2=&subnet[binding+1..];
    //   print!("{:#?}",&path);
    let path = String::from(path.into_os_string().to_str().unwrap());
    let path = format!("{}/ip_subnets/{}_{}",path,subnet_1,subnet_2);

    let mut _file = OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)
    .open(&path)
    .unwrap();
    for ip in ips{
    // fs::create_dir(format!("{}/ip_subnets/",path)).unwrap();
    if let Err(e) = writeln!(_file, "{ip}") {
        eprintln!("Couldn't write to file: {}", e);
    }
    }
}
use indicatif::ProgressBar;
use once_cell::sync::Lazy;
use std::io::{stdin, Write, Read};
fn get_ips()->Vec<String>{
    let mut ips :Vec<String> = vec![];
    let subnets = SUBNETS.clone();
    for subnet in subnets.into_iter(){
        let index = subnet.find(".").unwrap();
        let firs_octet:u8 = subnet[..index].parse().unwrap();
        //check subnet is in ok list 
        if CLOUD_FLARE_OK_LIST.iter().any(|&i|i==firs_octet ){
        let mut ips_in_sub=ip_fns::ips_in_subnet(String::from(subnet.clone()));
        // write_ip_subnets(&subnet, &  ips_in_sub);
        ips.append(&mut ips_in_sub);
        }
    }
    ips
}
fn read_ips_locally()->Vec<String>{
    let files_dir = fs::read_dir("./ip_subnets/").unwrap();
    let mut ips:Vec<String>=vec![];
    for file in files_dir{
        let mut file_content=String::new();
        let mut file = OpenOptions::new().read(true).open(file.unwrap().path()).unwrap();
        file.read_to_string(& mut file_content).unwrap();
        let ips_in_file = file_content.trim().lines();
        ips_in_file.into_iter().for_each(|ip|{
            ips.push(ip.trim().to_string())
        });
    }
    ips
}

fn main() {

    read_ips_locally();
    cyan_ln!("---------------------------");
    cyan_ln!("Cross Platform CfScanner ");
    cyan_ln!("By: BrPrS");
    cyan_ln!("Base Script by: Morteza Bashsiz");
    cyan_ln!("---------------------------");
    cyan_ln!("");
    let list_of_configs = find_configs();
    if list_of_configs.len()==0{
        red_ln!("No Config File in Server Folder");
        exit(1)
    }
    yellow_ln!("Choose Your Config:");

    for i in 0..list_of_configs.len(){
        yellow_ln!("{}-{}",i+1,list_of_configs[i])
    }
    yellow!(": ");
    let mut config_number = String::new();
    stdin().read_line(&mut config_number).unwrap();
    let config_number:u8 = match config_number.trim().parse::<u8>(){
        Ok(i)=>i-1,
        Err(_)=>{red_ln!("Failed");
                 exit(1)   }
    };
    let selected_config:String;
    if config_number<=(list_of_configs.len()+1) as u8{
        selected_config = list_of_configs[config_number as usize].clone();
    }
    else{
        red_ln!("Failed");
        exit(1);
    }
    yellow!("Enter number of threads: (default = 8): ");
    let mut number_of_thread = String::new();
    stdin().read_line(&mut number_of_thread).unwrap();

    if number_of_thread.trim().is_empty() {
        number_of_thread = "8".to_string()
    }
    let thread: usize = number_of_thread
        .trim()
        .parse()
        .expect("invalid input for thread number");

    let config_file = read_temp_config(&selected_config);
    let  ips:Vec<String>;
    
    let mut stdin_buf = String::new();
    let curl_installed = *CURL_INSTALLED;
    match env::consts::OS{
        "linux"=>{
            yellow!("Is nmap installed?");
            yellow!("ips will be loaded locally if nmap is not installed(y/N): ");
            stdin().read_line(&mut stdin_buf).unwrap();
            match stdin_buf.trim().to_lowercase().as_str() {
                "y"=>unsafe{ips = IPS.clone()},
                "n"|"" =>ips=read_ips_locally(),

                _=>{red_ln!("Invalid input");
                exit(1)        
                        }
            } 
        },
        "windows"=>ips=read_ips_locally(),
        _=>{red_ln!("unSupported OS");
            exit(1)        
                    }
    }
    
    let bar: ProgressBar = ProgressBar::new(ips.len() as u64);
    let mut handles: Vec<JoinHandle<()>> = vec![];
    bar.inc(0);

    for index in 0..ips.len() {
    let handle=each_thread_job(&ips[index], config_file.clone(), curl_installed.clone());
    
    handles.push(handle);

        if index % thread == 0 {
            for _ in 0..handles.len() {
                let handle = handles.pop().unwrap();
                    handle.join().unwrap();

                }
                bar.inc(thread as u64);
    
                kill_v2rays();
                kill_curls();
            }

            
        }
    bar.finish();
}

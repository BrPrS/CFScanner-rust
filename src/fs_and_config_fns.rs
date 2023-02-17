use datetime::{DatePiece, LocalDateTime, TimePiece};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc, Mutex};
static FILE: Lazy<Arc<Mutex<File>>> = Lazy::new(|| open_result_file());

pub fn write_ok_ips_in_results(ip: &String, respone_time: &String) {
    if let Err(e) = writeln!(FILE.lock().unwrap(), "{ip}   {respone_time}") {
        eprintln!("Couldn't write to file: {}", e);
    }
}
fn open_result_file() -> Arc<Mutex<File>> {
    let time = LocalDateTime::now();
    let time = format!("d{:?}-h{:?}", time.day(), time.hour());
    let result_dir_path:String;
    if cfg!(target_os="windows"){
        result_dir_path = format!(
            "{}\\result\\result_{:?}.txt",
            env::current_dir().unwrap().to_str().unwrap(),
            time
        ).replace("\"", "");
        
        println!("{}",result_dir_path);
    }
    else{
    result_dir_path = format!(
        "{}/result/result_{:?}.txt",
        env::current_dir().unwrap().to_str().unwrap(),
        time
    );
}
    //TODO add for create file if not existed
    let _file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&result_dir_path)
        .unwrap();

    let _file = Arc::new(Mutex::new(_file));

    _file
}


pub fn find_configs() -> Vec<String> {
    let path = env::current_dir().unwrap();
    //   print!("{:#?}",&path);
    let path = String::from(path.into_os_string().to_str().unwrap());
    //let path= path.strip_suffix("/src-tauri").unwrap();
    let path_url = format!("{}/servers/", path);
    // println!("{:#?}",path_url);
    let paths = fs::read_dir(&path_url).unwrap();
    let mut configs = Vec::new();
    for path in paths {
        let path_file = path.unwrap().path().display().to_string();
        if path_file.contains(".json") {
            // println!("{path_file}");
            let file_name = path_file
                .strip_prefix(&path_url)
                .unwrap()
                .strip_suffix(".json")
                .unwrap();
            configs.push(String::from(file_name));
        }
    }
    // println!("{:#?}",configs);
    configs
}

use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;
pub fn run_v2ray(config_name: &str) -> Child {
    //Debug
    // println!("{}",config_name);
    let path = env::current_dir().unwrap();
    let path = String::from(path.into_os_string().to_str().unwrap());
    let v2ray_path:String;
    let servers_path:String;
    if cfg!(target_os = "windows"){
        v2ray_path = format!("{}\\v2ray\\v2ray", path);
        servers_path = format!("{}\\configs\\", path);
    }
    else{
        v2ray_path = format!("{}/v2ray/v2ray", path);
        servers_path = format!("{}/configs/", path);
    }
    // println!("{check}");
    let configs_name = format!("{}{}", servers_path, config_name);
    // Debug
    //
    // blue_ln!("----------------------");
    // println!("{v2ray_path}");
    // println!("{servers_path}");
    // println!("{configs_name}");
    // blue_ln!("----------------------");
    let handle_v2ray_thread: Child;
    if cfg!(target_os = "windows") {
        handle_v2ray_thread = Command::new(format!("{}.exe",v2ray_path))
            .arg("-c")
            .arg(&configs_name)
            .stdout(Stdio::null())
            .spawn()
            .expect("failed to execute process");
        // println!("{:#?}",output);
    } else {
        handle_v2ray_thread = Command::new(v2ray_path)
            .arg("-c")
            .arg(&configs_name)
            .stdout(Stdio::null())
            .spawn()
            .expect("failed to execute process");
    };
    //Debug
    //   println!("v2ray started");

    thread::sleep(Duration::from_millis(500));
    handle_v2ray_thread
}

use crate::ip_fns::create_port_from_ip;
pub fn write_config_with_ip(template_config_file: Value, ip: &String) {
    let path = env::current_dir().unwrap();
    let mut config_file = template_config_file.clone();

    let port = create_port_from_ip(&ip);
    //Replacing values in order of ip and template config
    
    config_file["inbounds"][0]["port"] = format!("3{}", port).parse().unwrap();
    let _vmess_key_name = Value::String("vmess".to_string());
    let _trojan = Value::String("trojan".to_string());
    let protocol = config_file["outbounds"][0]["protocol"].clone().to_string();

    if protocol.contains("vmess") || protocol.contains("trojan") {
        if protocol.contains("vmess") {
            create_vmess_config_file(&mut config_file, &template_config_file, &ip);
        } else {

            create_trojan_config_file(&mut config_file, &template_config_file, &ip);
        }
    } else {
        println!("not supported protocol! exiting...");
        exit(1);
    }

    let out_path = format!(
        "{}/configs/config.json.{}",
        &path.to_str().unwrap(),
        ip.trim()
    );
    //Debug
    // println!("{}", out_path);
    std::fs::write(
        out_path,
        serde_json::to_string_pretty(&config_file).unwrap(),
    )
    .expect("failed to write temp config");
}

fn create_vmess_config_file(config_file: &mut Value, template_config_file: &Value, ip: &String) {
    config_file["outbounds"][0]["settings"]["vnext"][0]["address"] =
        serde_json::Value::String(ip.clone());
    config_file["outbounds"][0]["streamSettings"]["wsSettings"]["headers"]["Host"] =
        serde_json::Value::String(
            (&template_config_file["outbounds"][0]["streamSettings"]["wsSettings"]["headers"]
                ["Host"])
                .to_string(),
        );
    config_file["outbounds"][0]["streamSettings"]["wsSettings"]["headers"]["path"] =
        serde_json::Value::String(
            (&template_config_file["outbounds"][0]["streamSettings"]["wsSettings"]["headers"]
                ["path"])
                .to_string(),
        );
    config_file["outbounds"][0]["streamSettings"]["tlsSettings"]["serverName"] =
        template_config_file["outbounds"][0]["streamSettings"]["tlsSettings"]["serverName"].clone();

    config_file["outbounds"][0]["settings"]["vnext"][0]["users"] =
        template_config_file["outbounds"][0]["settings"]["vnext"][0]["users"].clone();
}

fn create_trojan_config_file(config_file: &mut Value, template_config_file: &Value, ip: &String) {
    config_file["outbounds"][0]["settings"]["servers"][0]["address"] =
        serde_json::Value::String(ip.clone());
    config_file["outbounds"][0]["streamSettings"]["tlsSettings"]["serverName"] = Value::String(
        template_config_file["outbounds"][0]["streamSettings"]["tlsSettings"]["serverName"]
            .to_string(),
    );

    config_file["outbounds"][0]["streamSettings"]["wsSettings"]["headers"]["Host"] =
        template_config_file["outbounds"][0]["streamSettings"]["wsSettings"]["headers"]["Host"]
            .clone();
}
pub fn read_temp_config(config_name:&String) -> Value {
    let path = env::current_dir().unwrap();
    //Debug
    //   println!("{:#?}", path.to_str().unwrap());
    let in_path = format!("{}/servers/{}.json", &path.to_str().unwrap(),config_name);

    let temp_config = fs::read_to_string(&in_path).expect("failed to open temp config file");
    let temp_config_json: Value =
        serde_json::from_str(&temp_config).expect("failed to load temp config file");

    temp_config_json
}

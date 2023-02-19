use colour::{blue, blue_ln, cyan_ln, green, green_ln, red, red_ln, yellow};
use std::time::Duration;
use serde_json::Value;
mod vmess_config_model;
mod fs_and_config_fns;
mod request_fns;
use crate::fs_and_config_fns::{read_temp_config, run_v2ray, write_config_with_ip};
mod ip_fns;
use crate::ip_fns::{check_ip};
use crate::request_fns::check_connection_through_v2ray;
use std::process::Command;
use std::thread;
use std::thread::JoinHandle;
/*
1-organize files
2-choose configs
----
find all ipis
----
3-run v2ray with selected config *
4-make request to server using v2ray
5-save result into file
6-make it run on multiple thread
7-handle errors/ show progress bar
-------
1- local list for nonmap command


-------------
8-use debug print

*/
static CURL_INSTALLED: Lazy<bool> = Lazy::new(|| true);

static CLOUD_FLARE_OK_LIST:Lazy<Vec<u8>>=Lazy::new(||vec![31, 45, 66, 80, 89, 103, 104, 108, 141, 147, 154, 159, 168, 170, 185, 188, 191, 192, 193,
194, 195, 199, 203, 205, 212]);
static mut IPS:Lazy<Vec<String>> = Lazy::new(||get_ips());
static SUBNETS: Lazy<Vec<String>> =  Lazy::new(||ip_fns::find_clf_ip_list());
    

use std::sync::{Arc, Mutex};

fn each_thread_job(ip: &String, config_file: Value) -> JoinHandle<()> {
    let ip = Arc::new(Mutex::new(ip.clone()));
    let config_file = Arc::new(Mutex::new(config_file));
    let handle = thread::spawn(move || {
        let ip = ip.lock().unwrap();
        let config_file = config_file.lock().unwrap();
        // if ip.contains("203.28.8") {
        if *CURL_INSTALLED{
        let (check_ip_reuslt, check_ip_handler) = check_ip(&ip);
        if check_ip_reuslt {
            write_config_with_ip(config_file.clone(), &ip);
            let mut handler = run_v2ray(format!("config.json.{}", ip.trim()).as_str());
            let check_connection_handler = check_connection_through_v2ray(*CURL_INSTALLED,&ip);
            handler.kill().unwrap();
            check_ip_handler.join().unwrap();
            if check_connection_handler.is_some(){
                check_connection_handler.unwrap().join().unwrap();
            }
    }
        }
        
    else{
        write_config_with_ip(config_file.clone(), &ip);
        let mut handler = run_v2ray(format!("config.json.{}", ip.trim()).as_str());
        let check_connection_handler = check_connection_through_v2ray(*CURL_INSTALLED,&ip);
        handler.kill().unwrap();
        if check_connection_handler.is_some(){
            check_connection_handler.unwrap().join().unwrap();
        }
    }
        // }
    });
    handle
}
fn kill_v2rays() {
    //for windows
    // taskkill /IM audiodg.exe /F
    if cfg!(target_os = "windows"){
        todo!()
    }
    else{
    Command::new("killall")
        .arg("v2ray")
        .output()
        .expect("failed to stop v2rays");
    }
}
fn kill_curls(){
    if cfg!(target_os = "windows"){
        todo!()
    }else{
    Command::new("killall")
    .arg("curl")
    .output()
    .expect("failed to stop v2rays");
    }
}
use std::env;
use std::fs::{self,File,OpenOptions};
// fn wtite_ip_subnets(subnet:&String,ips:&Vec<String>){
//     let path = env::current_dir().unwrap();
//     let path = format!("{:?}/ip_subnets/{}",path,subnet);
//     let file = OpenOptions::new().create(true).read(true).append(true).open(path).unwrap();
// }
//keeps number of thread always equal to given number
// fn thread_manager(handles:& mut Vec<JoinHandle<()>>,number_of_threads:&usize,ip: &String, config_file: Value){
//     if handles.len()<*number_of_threads{
//         let thread_handle = each_thread_job(ip, config_file);
//         handles.push(thread_handle);
//     }
//     for handle in handles{
//         if handle.join().is_ok(){
//             thread_manager(handles, number_of_threads, ip, config_file)
//         }
//     }
// }
use indicatif::ProgressBar;
use once_cell::sync::Lazy;
use std::io::stdin;
fn get_ips()->Vec<String>{
    let mut ips :Vec<String> = vec![];
    let subnets = SUBNETS.clone();
    for subnet in subnets.into_iter(){
        let index = subnet.find(".").unwrap();
        let firs_octet:u8 = subnet[..index].parse().unwrap();
        //check subnet is in ok list 
        if CLOUD_FLARE_OK_LIST.iter().any(|&i|i==firs_octet ){
        let mut ips_in_sub=ip_fns::ips_in_subnet(String::from(subnet));
        // wtite_ip_subnets()
        ips.append(&mut ips_in_sub);
        }
    }
    ips
}
fn main() {
    colour::cyan_ln!("---------------------------");
    cyan_ln!("Cross Platform CfScanner ");
    cyan_ln!("By: Prbarkati");
    cyan_ln!("Base script by: Morteza Bashsiz");
    cyan_ln!("---------------------------");
    cyan_ln!("");
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

    let config_file = read_temp_config();
    
    // for subnet in subnets{
    //     let index = subnet.find(".").unwrap();
    //     let firs_octet:u8 = subnet[..index].parse().unwrap();
    //     //check subnet is in ok list 
    //     if CLOUD_FLARE_OK_LIST.iter().any(|&i|i==firs_octet ){
    //     let mut ips_in_sub=ip_fns::ips_in_subnet(String::from(subnet));
    //     // wtite_ip_subnets()
    //     ips.append(&mut ips_in_sub);
    //     }
    // }
    let mut ips:Vec<String>;
    unsafe{ips = IPS.clone();}
    let bar: ProgressBar = ProgressBar::new(ips.len() as u64);
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // for index in 0..ips.len() {
        // thread_manager(& mut handles,&thread,&ips[index], config_file.clone());
    loop{
        if ips.len()==0{
            break;
        }
        let number_of_threads = &handles;
        if number_of_threads.len()<=thread{
        let handle = each_thread_job(&ips.pop().unwrap(), config_file.clone());

        handles.push(handle);
        }

        for handle_thread_index in 0..handles.len(){
            if handles[handle_thread_index].is_finished(){
                bar.inc(1 as u64);
                handles[handle_thread_index].join().unwrap();
                let handle =each_thread_job(&ips.pop().unwrap(), config_file.clone());
                handles.push(handle);
            }
        }
        // if index % thread == 0 {
        //     for _ in 0..handles.len() {
        //         let handle = handles.pop().unwrap();
        //         // handle.is_finished()
        //         handle.join().unwrap();
        //         kill_v2rays();
        //         kill_curls();
        //     }

            

        }
    bar.finish();

}
    // }

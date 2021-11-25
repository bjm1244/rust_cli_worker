use tokio::{self, process::Command};
use online::check;
use core::time;
use std::{option::Option, thread::sleep};

async fn view_msg_sleep(process_exec_flag: i32, checked_flag: i32, msg: &str)->Option<i32>{
    println!("{}", msg);
    if checked_flag == 0 {
        if process_exec_flag == 0 {
            let output = Command::new("echo").arg("The network is connected and program execute").output().await;
            match output {
                Ok(data) => {      
                    println!("The network is connected and program execute : {:?}", data);
                }
                Err(err) => {
                    println!("The network is connected and program execute error : {}", err);
                },
            };
            return Some(1);
        }else{
            println!("The network is connected and program running");
            return Some(process_exec_flag);
        }
    }else if checked_flag == 1 || checked_flag == 2 {
        if process_exec_flag == 0 {
            println!("The network is not connected and program do not execute");
            return Some(process_exec_flag);
        }else{
            let output = Command::new("echo").arg("The network is connected and program execute").output().await;
            match output {
                Ok(data) => {      
                    println!("The network is not connected and program exit : {:?}", data);
                }
                Err(err) => {
                    println!("The network is not connected and program exit error : {}", err);
                },
            };
            return Some(0);
        }
    }else{
        return None;
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut process_exec_flag: i32 = 0;
    loop {
        let checked_online = Option::Some(check(None).await.is_ok());
        match checked_online {
            Some(true) => {
                process_exec_flag = view_msg_sleep(process_exec_flag, 0, "connect").await.expect("Check None");
            },
            Some(false) => {
                process_exec_flag = view_msg_sleep(process_exec_flag, 1, "connect is dead false!!!").await.expect("Check None");
            },
            None => {
                // process_exec_flag = view_msg_sleep(process_exec_flag, 2, "connect is dead!!!").await.expect("Check None");
                break;
            },
        }
        println!("{}", process_exec_flag);
        sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

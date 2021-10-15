use tokio;
use online::check;
use core::time;
use std::{option::Option, thread::sleep};

async fn view_msg_sleep(process_exec_flag: i32, checked_flag: i32, msg: &str)->i32{
    println!("{}", msg);
    if checked_flag == 0 {
        if process_exec_flag == 0 {
            return 1;
        }else{
            return process_exec_flag;
        }
    }else if checked_flag == 1 || checked_flag == 2 {
        if process_exec_flag == 0 {
            return process_exec_flag;
        }else{
            return 0;
        }
    }else{
        if process_exec_flag == 0 {
            return process_exec_flag;
        }else{
            return 0;
        }
    }
}
#[tokio::main]
async fn main() {
    let mut process_exec_flag: i32 = 0;
    loop {
        let checked_online = Option::Some(check(None).await.is_ok());
        match checked_online {
            Some(true) => {
                process_exec_flag = view_msg_sleep(process_exec_flag, 0, "connect").await;
            },
            Some(false) => {
                process_exec_flag = view_msg_sleep(process_exec_flag, 1, "connect is dead false!!!").await;
            },
            None => {
                process_exec_flag = view_msg_sleep(process_exec_flag, 2, "connect is dead!!!").await;
            },
        }
        println!("{}", process_exec_flag);
        sleep(time::Duration::from_secs(1));
    }
}

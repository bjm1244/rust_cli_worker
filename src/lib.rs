
#[cfg(test)]
mod tests {
    use online::check;
    use tokio::process::Command;


    #[tokio::main]
    #[test]
    async fn test_check_internet_online() {
        let checked_online = Option::Some(check(None).await.is_ok());
        match checked_online {
            Some(true) => {
                println!("Return Some True : internet online");
            },
            Some(false) => {
                println!("Return Some False : internet offline");
            },
            None => {
                println!("Return None");
            },
        }
    }

    #[tokio::main]
    #[test]
    async fn test_exec_process_echo() {
        let output = Command::new("echo").arg("Unit test").output().await;
        match output {
            Ok(data) => {      
                println!("Success : {:?}", data);
            }
            Err(err) => {
                println!("error : {}", err);
            },
        };
    }
}
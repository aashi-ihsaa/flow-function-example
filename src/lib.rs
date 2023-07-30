use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let username = match qry.get("username") {
        Some(username_value) => {
            match username_value.as_str() {
                Some(username) => username,
                None => {
                    send_response(400, vec![], b"Bad Request: Invalid 'username' parameter.".to_vec());
                    return;
                }
            }
        }
        None => {
            send_response(400, vec![], b"Bad Request: Missing 'username' parameter.".to_vec());
            return;
        }
    };

    let password = match qry.get("password") {
        Some(password_value) => {
            match password_value.as_str() {
                Some(password) => password,
                None => {
                    send_response(400, vec![], b"Bad Request: Invalid 'password' parameter.".to_vec());
                    return;
                }
            }
        }
        None => {
            send_response(400, vec![], b"Bad Request: Missing 'password' parameter.".to_vec());
            return;
        }
    };

    
    let login_success = username == "admin" && password == "admin";

    let resp = if login_success {
        "Login Successful!".to_string()
    } else {
        "Login Failed: Invalid credentials.".to_string()
    };

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/plain"))],
        resp.as_bytes().to_vec(),
    );
}

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

    // Your flow function implementation here
    // ...

    let resp = "Flow function response".to_string();

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/plain"))],
        resp.into_bytes(),
    );
}

fn main() {
    // Start the execution of the flow function.
    // In this case, we use the run() function.
    run().unwrap();
}

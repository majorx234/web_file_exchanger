use anyhow::Result;
use http::Method;
use httpc_test;
use reqwest::{Body, Response};
use serde_json::{json, Value};

#[tokio::test]
async fn getpost_helper() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;

    let res = hc.do_post(
        "/login",
        json!({"user_name" : "Heinz", "password_hash": "f4d3ad4f524a2c260f3220d954abb08b7953a9a3998fd46a8a221c2bb2acf3c6"}),
    ).await?;
    res.print().await?;
    let token = &res.json_body().unwrap()["token"];
    let mut bearer_token = "Bearer ".to_string() + &token.to_string()[1..];
    bearer_token.pop();
    // need better testing lib to provide token information in header, switch to reqwest

    let client = reqwest::Client::builder().build()?;

    let url_info = "http://localhost:8080/info?info=foo".to_string();
    let reqwest_res_info = client
        .get(&url_info)
        .header("Authorization", &bearer_token)
        .send()
        .await?;

    let status = reqwest_res_info.status();

    let json_body = reqwest_res_info.json::<Value>().await?;
    println!("status: {}, info: {}", status, json_body);

    let url_files = "http://localhost:8080/files".to_string();
    let reqwest_res_files = client
        .get(&url_files)
        .header("Authorization", &bearer_token)
        .send()
        .await?;
    let status = reqwest_res_files.status();

    let json_body = reqwest_res_files.json::<Value>().await?;
    println!("status: {}, files: {}", status, json_body);

    //hc.do_get("/files").await?.print().await?;

    Ok(())
}

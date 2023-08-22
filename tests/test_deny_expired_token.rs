use anyhow::Result;
use http::Method;
use httpc_test;
use reqwest::{Body, Response};
use serde_json::{json, Value};

#[tokio::test]
async fn test_deny_expired_token() -> Result<()> {
    // test deny old token
    let old_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjoiSGVpbnoiLCJleHAiOjE2OTI2NTQ0ODJ9.ED_Ymwe2rxkqCXhbYXv3_pAAysxSy7_2hNt4fpq0G84";

    let bearer_old_token = "Bearer ".to_string() + old_token;
    let url_files = "http://localhost:8080/files".to_string();

    let client = reqwest::Client::builder().build()?;

    let reqwest_res_files = client
        .get(&url_files)
        .header("Authorization", &bearer_old_token)
        .send()
        .await?;
    let status = reqwest_res_files.status();
    assert_eq!(status, 403);
    let json_body = reqwest_res_files.json::<Value>().await?;
    println!("status: {}, files: {}", status, json_body);
    Ok(())
}

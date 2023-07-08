use anyhow::Result;
use httpc_test;
use serde_json::json;

#[tokio::test]
async fn getpost_helper() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/info?info=foo").await?.print().await?;
    hc.do_post(
        "/login",
        json!({"user_name" : "Horst", "password_hash": "123456789"}),
    )
    .await?
    .print()
    .await?;
    Ok(())
}

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
        json!({"user_name" : "Heinz", "password_hash": "f4d3ad4f524a2c260f3220d954abb08b7953a9a3998fd46a8a221c2bb2acf3c6"}),
    )
    .await?
    .print()
    .await?;

    Ok(())
}

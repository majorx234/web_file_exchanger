use anyhow::Result;
use httpc_test;

#[tokio::test]
async fn getpost_helper() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}

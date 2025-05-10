#![allow(unused)]
use std::io;
use std::collections::HashMap;

#[tokio::test]
async fn qdev() -> httpc_test::Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;
	let req_get = hc.do_get(
        "/"
    );
	req_get.await?.print().await?;
	Ok(())
}
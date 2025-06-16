#![allow(unused)]
use std::error::Error;
use std::io;
use std::collections::HashMap;
use rusite::utils;

// #[tokio::test]
// async fn qdev() -> httpc_test::Result<()> {
// 	let hc = httpc_test::new_client("http://localhost:8000")?;
// 	// let req_get = hc.do_get(
//     //     "/static/posts/2025-05-12%2001:25:20_test0.md"
//     // );
// 	let req_get = hc.do_get(
//         "/api/v1/article/content/path?aid=ea38c503-2e8c-11f0-8936-f8e43b81de7b"
//     );
// 	req_get.await?.print().await?;
// 	Ok(())
// }

#[tokio::test]
async fn get_cpu_usage() -> Result<(), Box<dyn Error>> {
	match utils::get_cpu_usage().await {
		Ok(cpu_usage) => {
			println!("CPU usage: {}", cpu_usage);
		},
		Err(err) => {
			eprintln!("Error: {}", err);
		}
	}
	Ok(())
}
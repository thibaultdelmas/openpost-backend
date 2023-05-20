#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    hc.do_get("/api/healthchecker").await?.print().await?;

	let req_signup = hc.do_post(
		"/api/auth/register",
		json!(
			{
                "email": "admin@admin.com",
                "name": "Admin",
                "password": "password123"
            }
		),
	);
    req_signup.await?.print().await?;
    let req_login = hc.do_post(
		"/api/login",
		json!({
            "email": "admin@admin.com",
            "password": "password123"
		}),
	);
	req_login.await?.print().await?;

    hc.do_get("/api/users/me").await?.print().await?;

    hc.do_get("/api/logout").await?.print().await?;
    
    hc.do_get("/api/users/me").await?.print().await?;
    
    Ok(())

}
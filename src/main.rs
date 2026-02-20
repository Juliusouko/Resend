use dotenvy::dotenv;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("RESEND_API_KEY").expect("API KEY not found");

    let resend = Resend::new(&api_key);

    let email = vec![
        CreateEmailBaseOptions::new(
            "Acme <updates@aswito.com>",
            vec!["oukojulius224@gmail.com"],
            "Test Email",
        )
        .with_html("<p>This is a test email on how to send multiple emails</p>"),
    ];

    let email = resend.batch.send(email).await?;

    Ok(())
}

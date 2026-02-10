use email::{
    account::config::{
        oauth2::{OAuth2Config, OAuth2Method, OAuth2Scopes},
        AccountConfig,
    },
    backend::context::BackendContextBuilder,
    outlook::{OutlookConfig, OutlookContextBuilder},
};
use secret::Secret;
use std::sync::Arc;

// Run with: cargo run --example outlook_oauth --features outlook,oauth2
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure OAuth2 for Microsoft
    let oauth2_config = OAuth2Config {
        method: OAuth2Method::XOAuth2,
        client_id: "YOUR_CLIENT_ID".to_string(),
        client_secret: None, // Public client doesn't need secret
        auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
        access_token: Secret::default(),
        refresh_token: Secret::default(),
        pkce: true,
        redirect_scheme: Some("http".to_string()),
        redirect_host: Some("localhost".to_string()),
        redirect_port: Some(8080),
        scopes: OAuth2Scopes::Scopes(vec![
            "https://graph.microsoft.com/Mail.Read".to_string(),
            "https://graph.microsoft.com/Mail.ReadWrite".to_string(),
            "offline_access".to_string(),
        ]),
    };

    // Create Outlook config
    let outlook_config = Arc::new(OutlookConfig {
        client_id: "YOUR_CLIENT_ID".to_string(),
        tenant_id: "common".to_string(),
        auth: email::outlook::config::OutlookAuthConfig::OAuth2(oauth2_config.clone()),
    });

    // Create account config
    let account_config = Arc::new(AccountConfig {
        name: "test".to_string(),
        email: "user@example.com".to_string(),
        ..Default::default()
    });

    println!("Starting OAuth 2.0 Authorization Code Flow...");
    println!();
    println!("This will:");
    println!("1. Generate an authorization URL");
    println!("2. Open it in your browser (or you can copy/paste it)");
    println!("3. Start a local server on http://localhost:8080 to receive the callback");
    println!();

    // Run the OAuth flow - this will print the URL and wait for callback
    oauth2_config
        .configure(|| {
            // If client secret is needed, prompt for it
            println!("Enter client secret (or press Enter if not needed):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            Ok(input.trim().to_string())
        })
        .await?;

    println!();
    println!("✓ Authorization successful!");
    println!();

    // Now build the context with the obtained token
    let mut builder = OutlookContextBuilder::new(account_config, outlook_config);
    builder.prebuild_token().await?;
    let _context = builder.build().await?;

    println!("✓ Outlook context built successfully!");

    Ok(())
}

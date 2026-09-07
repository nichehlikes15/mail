use rand::{Rng, distr::Alphanumeric};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Clone, Debug)]
pub struct Email {
    pub id: String,
    pub from: String,
    pub subject: String,
    pub intro: String,
    pub seen: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
struct MessagesResponse {
    #[serde(rename = "hydra:member")]
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct Message {
    id: String,
    from: MessageFrom,
    subject: String,
    intro: Option<String>,
    seen: bool,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct MessageFrom {
    address: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Clone, Debug)]
pub struct TempEmail {
    pub address: String,
    pub password: String,
    pub id: String,
    pub token: String
}

#[derive(Debug, Deserialize)]
struct AccountResponse {
    id: String,
    address: String,
}

fn random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub async fn create_account() -> Result<TempEmail, Box<dyn std::error::Error>> {
    let client = Client::new();

    let username = random_string(12).to_lowercase();
    let password = random_string(20);

    let domains_response = client.get("https://api.mail.tm/domains").send().await?;
    let domains: serde_json::Value = domains_response.json().await?;
    let domain = domains["hydra:member"][0]["domain"]
        .as_str()
        .ok_or("No domain available")?;

    let address = format!("{}@{}", username, domain);

    let response = client
        .post("https://api.mail.tm/accounts")
        .json(&json!({"address": address,"password": password}))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;

        return Err(format!("Failed to create account: {} - {}", status, body).into());
    }



    let token_response = client
        .post("https://api.mail.tm/token")
        .json(&json!({
            "address": address,
            "password": password
        }))
        .send()
        .await?;

    if !token_response.status().is_success() {
        let status = token_response.status();
        let body = token_response.text().await?;

        return Err(format!(
            "Failed to login to Mail.tm: {} - {}",
            status,
            body
        )
        .into());
    }

    let account: AccountResponse = response.json().await?;

    println!(
        "Temporary email created: {}",
        account.address
    );

    println!(
        "Account ID: {}",
        account.id
    );

    let token: TokenResponse = token_response
        .json()
        .await?;

    println!("Mail.tm token acquired");

    Ok(TempEmail {
        address,
        password,
        id: account.id,
        token: token.token,
    })
}

async fn get_token(
    client: &Client,
    email: &TempEmail,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.mail.tm/token")
        .json(&json!({
            "address": email.address,
            "password": email.password
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to login to Mail.tm: {} - {}",
            response.status(),
            response.text().await?
        )
        .into());
    }

    let token: TokenResponse = response.json().await?;

    Ok(token.token)
}

pub async fn get_mail(email: &TempEmail) -> Result<Vec<Email>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let token = get_token(&client, email).await?;

    let response = client
        .get("https://api.mail.tm/messages")
        .bearer_auth(&token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to retrieve mail: {} - {}",
            response.status(),
            response.text().await?
        )
        .into());
    }

    let messages: MessagesResponse = response.json().await?;

    let emails = messages
        .messages
        .into_iter()
        .map(|message| Email {
            id: message.id,
            from: message.from.address,
            subject: message.subject,
            intro: message.intro.unwrap_or_default(),
            seen: message.seen,
            created_at: message.created_at,
        })
        .collect();

    Ok(emails)
}

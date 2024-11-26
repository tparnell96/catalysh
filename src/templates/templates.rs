use clap::{Args, Parser, Subcommand};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use crate::config::Config;
use crate::api::auth::Token;

// Root command template
#[derive(Debug, Parser)]
#[command(name = "catsh", about = "A REPL CLI for managing network devices")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Main command structure
#[derive(Debug, Subcommand)]
pub enum Commands {
    ExampleCommand {
        #[command(subcommand)]
        subcommand: ExampleSubcommands,
    },
}

// Subcommands template
#[derive(Debug, Subcommand)]
pub enum ExampleSubcommands {
    List,
    Details {
        #[arg(help = "ID of the item to retrieve details for")]
        id: String,
    },
    Create(NewItemArgs),
}

// Arguments for a subcommand
#[derive(Debug, Args)]
pub struct NewItemArgs {
    #[arg(help = "Name of the new item")]
    pub name: String,
    #[arg(help = "Description of the new item")]
    pub description: String,
}

// Handle ExampleCommand
pub async fn handle_example_command(config: Config, token: Token, subcommand: ExampleSubcommands) -> Result<()> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    match subcommand {
        ExampleSubcommands::List => {
            let response = api_get::<Vec<Item>>(&client, &config, &token, "example/list").await?;
            println!("Items: {:?}", response);
        }
        ExampleSubcommands::Details { id } => {
            let response = api_get::<Item>(&client, &config, &token, &format!("example/details/{}", id)).await?;
            println!("Item Details: {:?}", response);
        }
        ExampleSubcommands::Create(args) => {
            let new_item = NewItem {
                name: args.name,
                description: args.description,
            };
            let response = api_post::<Item, NewItem>(&client, &config, &token, "example/create", &new_item).await?;
            println!("Created Item: {:?}", response);
        }
    }

    Ok(())
}

// Example Data Structures
#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct NewItem {
    pub name: String,
    pub description: String,
}

// API Call Templates
pub async fn api_get<T: Deserialize<'static>>(
    client: &Client,
    config: &Config,
    token: &Token,
    endpoint: &str,
) -> Result<T> {
    let url = format!("{}/{}", config.dnac_url, endpoint);
    let mut resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .send()
        .await?;

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(anyhow!("Unauthorized: Token may have expired"));
    }

    if !resp.status().is_success() {
        return Err(anyhow!("GET request failed: {}", resp.status()));
    }

    let result = resp.json::<T>().await?;
    Ok(result)
}

pub async fn api_post<T: Deserialize<'static>, U: Serialize>(
    client: &Client,
    config: &Config,
    token: &Token,
    endpoint: &str,
    body: &U,
) -> Result<T> {
    let url = format!("{}/{}", config.dnac_url, endpoint);
    let mut resp = client
        .post(&url)
        .header("X-Auth-Token", &token.value)
        .json(body)
        .send()
        .await?;

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(anyhow!("Unauthorized: Token may have expired"));
    }

    if !resp.status().is_success() {
        return Err(anyhow!("POST request failed: {}", resp.status()));
    }

    let result = resp.json::<T>().await?;
    Ok(result)
}

pub async fn api_put<T: Deserialize<'static>, U: Serialize>(
    client: &Client,
    config: &Config,
    token: &Token,
    endpoint: &str,
    body: &U,
) -> Result<T> {
    let url = format!("{}/{}", config.dnac_url, endpoint);
    let mut resp = client
        .put(&url)
        .header("X-Auth-Token", &token.value)
        .json(body)
        .send()
        .await?;

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(anyhow!("Unauthorized: Token may have expired"));
    }

    if !resp.status().is_success() {
        return Err(anyhow!("PUT request failed: {}", resp.status()));
    }

    let result = resp.json::<T>().await?;
    Ok(result)
}

pub async fn api_delete<T: Deserialize<'static>>(
    client: &Client,
    config: &Config,
    token: &Token,
    endpoint: &str,
) -> Result<T> {
    let url = format!("{}/{}", config.dnac_url, endpoint);
    let mut resp = client
        .delete(&url)
        .header("X-Auth-Token", &token.value)
        .send()
        .await?;

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(anyhow!("Unauthorized: Token may have expired"));
    }

    if !resp.status().is_success() {
        return Err(anyhow!("DELETE request failed: {}", resp.status()));
    }

    let result = resp.json::<T>().await?;
    Ok(result)
}

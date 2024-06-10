use clap::{Parser, Subcommand, ValueEnum};
use webhook::{send_discord_webhook, webhook::{WebhookConfig, WebhookError}};
use serde_json::to_vec;
//use std::process;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Config {
        /// Webhook URL to use
        #[arg(short, long)]
        webhook_url: String,

        /// Optional username to display
        #[arg(short, long)]
        username: Option<String>,

        /// Optional avatar URL to display
        #[arg(short, long)]
        avatar_url: Option<String>,
    },
    Send {
        /// Content of the message to send
        #[arg(short, long)]
        content: Option<String>,
    },
    Update {
        /// Webhook URL to use
        #[arg(short, long)]
        webhook_url: String,

        /// Optional username to display
        #[arg(short, long)]
        username: Option<String>,

        /// Optional avatar URL to display
        #[arg(short, long)]
        avatar_url: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum WebhookType {
    Discord,
    Slack,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Config { webhook_url, username, avatar_url, .. } => {
            match config_webhook(webhook_url, username, avatar_url).await {
                Ok(_) => println!("Webhook configuration saved successfully."),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::Send { content } => {
            match send_webhook(content).await {
                Ok(_) => println!("Message sent successfully."),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::Update { webhook_url, username, avatar_url, .. } => {
            match config_webhook(webhook_url, username, avatar_url).await {
                Ok(_) => println!("Webhook configuration updated successfully."),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        //_exit => process::exit(1),
    }
}

async fn config_webhook(
    webhook_url: String,
    username: Option<String>,
    avatar_url: Option<String>,
) -> Result<(), WebhookError> {
    let config = WebhookConfig {
        webhook_url,
        username,
        avatar_url,
        content: Some("You have configured the webhook".to_string()),
    };

    // Handle webhook_type here if necessary

    // Save the config to a file
    let config_json = to_vec(&config).map_err(|e| WebhookError::InvalidResponse(e.to_string()))?;

    let mut file = File::create("webhook_config.json").await
        .map_err(|e| WebhookError::InvalidResponse(e.to_string()))?;
    file.write_all(&config_json).await
        .map_err(|e| WebhookError::InvalidResponse(e.to_string()))?;

    // Send the webhook
    send_discord_webhook(&config).await
}

async fn send_webhook(content: Option<String>) -> Result<(), WebhookError> {
    // Read the webhook configuration from the file
    let mut file = File::open("webhook_config.json").await
        .map_err(|e| WebhookError::InvalidResponse(e.to_string()))?;

    let mut config_json = Vec::new();
    file.read_to_end(&mut config_json).await
        .map_err(|e| WebhookError::InvalidResponse(e.to_string()))?;

    let mut config: WebhookConfig = serde_json::from_slice(&config_json)
        .map_err(|e| WebhookError::InvalidResponse(e.to_string()))?;

    config.content = content;

    send_discord_webhook(&config).await
}

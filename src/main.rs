mod cli;
mod telegram;
mod utils;
use core::panic;

use clap::Parser;
use grammers_client::types::Chat;

#[tokio::main]
async fn main() {
    let cli_args = cli::Cli::parse();

    let client = telegram::TgCliClient::connect(
        cli_args.api_id,
        cli_args.api_hash,
        cli_args.session_file.clone(),
    )
    .await;

    match cli_args.command {
        cli::Commands::Login { phone, password } => {
            println!("Logging in {}", phone);
            let me = client.login(phone, password).await.unwrap();
            println!("Logged in as '{}'", me.full_name());
        }
        cli::Commands::ListDialogs { filter } => {
            if !client.is_authorized().await {
                panic!("Not logged in - consider invoking login command first");
            }
            let dialogs = client.get_dialogs_by_name(filter).await;
            println!("Listing {} dialogs:", dialogs.len());
            for dialog in dialogs {
                let prefix = match dialog.chat {
                    Chat::User(_) => "User",
                    Chat::Group(_) => "Group",
                    Chat::Channel(_) => "Channel",
                };
                println!(
                    "{} {} {} (@{})",
                    prefix,
                    dialog.chat().id(),
                    dialog.chat().name(),
                    dialog.chat().username().unwrap_or("-")
                );
            }
        }
        cli::Commands::Msg {
            dialog,
            numeric_id,
            message,
        } => {
            if !client.is_authorized().await {
                panic!("Not logged in - consider invoking login command first");
            }

            let dialog_id: i64;

            if numeric_id {
                dialog_id = dialog.parse().unwrap();
            } else {
                let found_dialogs = client.get_dialogs_by_name(dialog.clone()).await;
                if found_dialogs.len() < 1 {
                    panic!("'{}' matched no dialogs", dialog);
                } else if found_dialogs.len() > 1 {
                    panic!("'{}' matched more than one dialog", dialog);
                }
                dialog_id = found_dialogs[0].chat().id();
            }

            client.send_message(dialog_id, message).await;
        }
    }

    client.save_session().await
}

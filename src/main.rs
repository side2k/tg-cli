mod cli;
mod telegram;
mod utils;
use std::process;

use crate::telegram::SessionSaver;
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
    .await
    .unwrap_or_else(|error| {
        eprint!("{}", error);
        process::exit(1)
    });

    if let cli::Commands::Login { phone, password } = cli_args.command {
        println!("Logging in {}", phone);
        match client.login(phone, password).await {
            Ok(client) => println!("Logged in as '{}'", client.user.full_name()),
            Err(error) => eprintln!("{}", error),
        }
    } else {
        // process commands that require TgCliLoggedInClient
        let client = client.authorized().await.unwrap_or_else(|error| {
            eprintln!("{}", error);
            process::exit(1);
        });

        match cli_args.command {
            cli::Commands::Login {
                phone: _,
                password: _,
            } => {} // login command is processed above

            cli::Commands::ListDialogs { filter } => {
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
                let dialog_id: i64;

                if numeric_id {
                    dialog_id = dialog.parse().unwrap();
                } else {
                    let found_dialogs = client.get_dialogs_by_name(dialog.clone()).await;
                    if found_dialogs.len() < 1 {
                        eprintln!("'{}' matched no dialogs", dialog);
                        process::exit(1);
                    } else if found_dialogs.len() > 1 {
                        eprintln!("'{}' matched more than one dialog", dialog);
                        process::exit(1);
                    }
                    dialog_id = found_dialogs[0].chat().id();
                }

                match client.send_message(dialog_id, message).await {
                    Ok(message) => println!("Message {} sent", message.id()),
                    Err(error) => eprintln!("Error sending message: {}", error),
                }
            }
        }
    }

    match client.save_session().await {
        Err(error) => eprintln!("Couldn't save session: {}", error),
        Ok(_) => {}
    }
}

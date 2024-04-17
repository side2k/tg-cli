mod cli;
mod telegram;
mod utils;
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
            let mut dialogs = client.get_dialogs().await;

            println!("Listing {} dialogs:", dialogs.len());
            if filter.len() > 0 {
                if filter.starts_with("@") {
                    dialogs = dialogs
                        .into_iter()
                        .filter(|dialog| {
                            dialog.chat().username().unwrap_or("").to_lowercase()
                                == filter.to_lowercase().trim_start_matches("@")
                        })
                        .collect();
                } else {
                    dialogs = dialogs
                        .into_iter()
                        .filter(|dialog| {
                            dialog
                                .chat()
                                .name()
                                .to_lowercase()
                                .contains(filter.to_lowercase().as_str())
                        })
                        .collect();
                }
            }
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
        cli::Commands::Msg { dialog_id, message } => {
            if !client.is_authorized().await {
                panic!("Not logged in - consider invoking login command first");
            }

            client.send_message(dialog_id, message).await;
        }
    }

    client.save_session().await
}

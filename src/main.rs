mod cli;
mod telegram;
mod utils;
use clap::Parser;
use grammers_client::types::Chat;

#[tokio::main]
async fn main() {
    let cli_args = cli::Cli::parse();

    let session = telegram::get_session(cli_args.session_file.clone());
    let client = telegram::get_client(cli_args.api_id, cli_args.api_hash, session).await;

    match cli_args.command {
        cli::Commands::Login { phone, password } => {
            println!("Logging in {}", phone);
            let me = telegram::login(&client, phone, password).await.unwrap();
            println!("Logged in as '{}'", me.full_name());
        }
        cli::Commands::ListDialogs {} => {
            if !client.is_authorized().await.unwrap() {
                panic!("Not logged in - consider invoking login command first");
            }
            let mut dialogs = client.iter_dialogs();
            println!("Listing {} dialogs:", dialogs.total().await.unwrap());
            while let Some(dialog) = dialogs.next().await.unwrap() {
                let prefix = match dialog.chat {
                    Chat::User(_) => "User",
                    Chat::Group(_) => "Group",
                    Chat::Channel(_) => "Channel",
                };
                println!("{} {} {}", prefix, dialog.chat().id(), dialog.chat().name());
            }
        }
    }

    client
        .session()
        .save_to_file(cli_args.session_file)
        .unwrap();
}

mod cli;
mod telegram;
mod utils;
use clap::Parser;

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
    }

    client
        .session()
        .save_to_file(cli_args.session_file)
        .unwrap();
}

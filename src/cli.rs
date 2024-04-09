use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = ".session")]
    pub session_file: String,
    #[arg(long, env = "API_ID")]
    pub api_id: i32,
    #[arg(long, env = "API_HASH")]
    pub api_hash: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Logs into the telegram account, saving session file
    Login {
        /// phone number, in international format. E.g.: +79012345678
        #[arg(short, long, env = "TGCLI_PHONE")]
        phone: String,
        #[arg(long, env = "TGCLI_PASSWORD")]
        password: Option<String>,
    },
    /// List all the dialogs and their ids
    ListDialogs {},
}